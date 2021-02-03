/*
 * Copyright © 2009-2020 Frictional Games
 * 
 * This file is part of Amnesia: The Dark Descent.
 * 
 * Amnesia: The Dark Descent is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version. 

 * Amnesia: The Dark Descent is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with Amnesia: The Dark Descent.  If not, see <https://www.gnu.org/licenses/>.
 */

#include "scene/Entity3D.h"
#include "scene/Node3D.h"
#include "math/Math.h"
#include "system/LowLevelSystem.h"
#include <utility>

namespace hpl {

  //////////////////////////////////////////////////////////////////////////
  // CONSTRUCTORS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------
  iEntity3D::iEntity3D(tString asName) {
    msName                 = std::move(asName);
    mbIsActive             = true;
    mpParentNode           = nullptr;
    m_mtxLocalTransform    = cMatrixf::Identity;
    m_mtxWorldTransform    = cMatrixf::Identity;
    mbTransformUpdated     = true;
    mlCount                = 0;
    msSourceFile           = "";
    mbApplyTransformToBV   = true;
    mbUpdateBoundingVolume = true;
    mpParent               = nullptr;
    mlIteratorCount        = -1;
    mbIsSaved              = true;
    mlUniqueID             = -1;
  }

  iEntity3D::~iEntity3D() {
    if (mpParentNode) {
      mpParentNode->RemoveEntity(this);
    } else if (mpParent) {
      mpParent->RemoveChild(this);
    }

    for (auto* child : mlstChildren) {
      child->mpParent = nullptr;
    }

    for (auto* node : mlstNodeChildren) {
      node->mpEntityParent = nullptr;
    }
  }

  //-----------------------------------------------------------------------

  //////////////////////////////////////////////////////////////////////////
  // PUBLIC METHODS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------

  cVector3f iEntity3D::GetLocalPosition() {
    return m_mtxLocalTransform.GetTranslation();
  }

  //-----------------------------------------------------------------------

  cMatrixf& iEntity3D::GetLocalMatrix() {
    return m_mtxLocalTransform;
  }

  //-----------------------------------------------------------------------

  cVector3f iEntity3D::GetWorldPosition() {
    UpdateWorldTransform();
    return m_mtxWorldTransform.GetTranslation();
  }

  //-----------------------------------------------------------------------

  cMatrixf& iEntity3D::GetWorldMatrix() {
    UpdateWorldTransform();
    return m_mtxWorldTransform;
  }

  //-----------------------------------------------------------------------

  void iEntity3D::SetPosition(const cVector3f& avPos) {
    m_mtxLocalTransform.m[0][3] = avPos.x;
    m_mtxLocalTransform.m[1][3] = avPos.y;
    m_mtxLocalTransform.m[2][3] = avPos.z;
    SetTransformUpdated();
  }

  //-----------------------------------------------------------------------

  void iEntity3D::SetMatrix(const cMatrixf& a_mtxTransform) {
    m_mtxLocalTransform = a_mtxTransform;
    SetTransformUpdated();
  }

  //-----------------------------------------------------------------------

  void iEntity3D::SetWorldPosition(const cVector3f& avWorldPos) {
    if (mpParent) {
      SetPosition(avWorldPos - mpParent->GetWorldPosition());
    } else {
      SetPosition(avWorldPos);
    }
  }

  //-----------------------------------------------------------------------

  void iEntity3D::SetWorldMatrix(const cMatrixf& a_mtxWorldTransform) {
    if (mpParent) {
      auto relative_matrix = cMath::MatrixMul(cMath::MatrixInverse(mpParent->GetWorldMatrix()),
                                              a_mtxWorldTransform);
      SetMatrix(relative_matrix);
    } else {
      SetMatrix(a_mtxWorldTransform);
    }
  }

  //-----------------------------------------------------------------------

  void iEntity3D::SetTransformUpdated(bool abUpdateCallbacks) {
    mbTransformUpdated = true;
    mlCount++;
    mbUpdateBoundingVolume = true;

    OnTransformUpdated();

    //Update children
    for (auto* child : mlstChildren) {
      child->SetTransformUpdated(true);
    }

    //Update node children
    for (auto* node : mlstNodeChildren) {
      node->SetWorldTransformUpdated();
    }

    //Update callbacks
    if (!mlstCallbacks.empty() && abUpdateCallbacks) {
      for (auto* callback : mlstCallbacks) {
        callback->OnTransformUpdate(this);
      }
    }
  }

  //-----------------------------------------------------------------------

  bool iEntity3D::GetTransformUpdated() {
    return mbTransformUpdated;
  }

  //-----------------------------------------------------------------------

  int iEntity3D::GetTransformUpdateCount() {
    return mlCount;
  }

  //-----------------------------------------------------------------------

  cBoundingVolume* iEntity3D::GetBoundingVolume() {
    if (mbApplyTransformToBV && mbUpdateBoundingVolume) {
      mBoundingVolume.SetTransform(GetWorldMatrix());
      mbUpdateBoundingVolume = false;
    }

    return &mBoundingVolume;
  }

  //-----------------------------------------------------------------------

  void iEntity3D::AddCallback(iEntityCallback* apCallback) {
    mlstCallbacks.push_back(apCallback);
  }

  //-----------------------------------------------------------------------

  void iEntity3D::RemoveCallback(iEntityCallback* apCallback) {
    STLFindAndRemove(mlstCallbacks, apCallback);
  }

  //-----------------------------------------------------------------------

  void iEntity3D::AddChild(iEntity3D* apEntity) {
    if (apEntity == nullptr) {
      return;
    }
    if (apEntity->mpParent != nullptr) {
      apEntity->mpParent->RemoveChild(apEntity);
    }

    mlstChildren.push_back(apEntity);
    apEntity->mpParent = this;

    apEntity->SetTransformUpdated(true);
  }

  void iEntity3D::RemoveChild(iEntity3D* apEntity) {
    for (auto it = mlstChildren.begin(); it != mlstChildren.end();) {
      iEntity3D* child = *it;
      if (child == apEntity) {
        child->mpParent = nullptr;
        it              = mlstChildren.erase(it);
      } else {
        ++it;
      }
    }
  }

  bool iEntity3D::IsChild(iEntity3D* apEntity) {
    return std::any_of(mlstChildren.begin(),
                       mlstChildren.end(),
                       [=](auto& a) { return apEntity == a; });
  }

  iEntity3D* iEntity3D::GetEntityParent() {
    return mpParent;
  }

  cEntity3DIterator iEntity3D::GetChildIterator() {
    return cEntity3DIterator(&mlstChildren);
  }

  //-----------------------------------------------------------------------

  void iEntity3D::AddNodeChild(cNode3D* apNode) {
    if (apNode->mpEntityParent != nullptr) {
      apNode->mpEntityParent->RemoveNodeChild(apNode);
    }

    mlstNodeChildren.push_back(apNode);
    apNode->mpEntityParent = this;

    apNode->SetWorldTransformUpdated();
  }

  void iEntity3D::RemoveNodeChild(cNode3D* apNode) {
    for (auto it = mlstNodeChildren.begin(); it != mlstNodeChildren.end();) {
      cNode3D* node = *it;
      if (node == apNode) {
        node->mpEntityParent = nullptr;
        it                   = mlstNodeChildren.erase(it);
      } else {
        ++it;
      }
    }
  }

  bool iEntity3D::IsNodeChild(cNode3D* apNode) {
    return std::any_of(mlstNodeChildren.begin(),
                       mlstNodeChildren.end(),
                       [=](auto& a) { return apNode == a; });
  }

  //-----------------------------------------------------------------------

  //////////////////////////////////////////////////////////////////////////
  // PRIVATE METHODS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------
  void iEntity3D::UpdateWorldTransform() {
    if (mbTransformUpdated) {
      //Log("CREATING Entity '%s' world transform!\n",msName.c_str());

      mbTransformUpdated = false;

      //first check if there is a node parent
      if (mpParentNode) {
        auto* node3D        = static_cast<cNode3D*>(mpParentNode);
        m_mtxWorldTransform = cMath::MatrixMul(node3D->GetWorldMatrix(), m_mtxLocalTransform);
      }

      //If there is no node parent check for entity parent
      else if (mpParent) {
        m_mtxWorldTransform = cMath::MatrixMul(mpParent->GetWorldMatrix(), m_mtxLocalTransform);
      } else {
        m_mtxWorldTransform = m_mtxLocalTransform;
      }
    }
  }

  //-----------------------------------------------------------------------
} // namespace hpl
