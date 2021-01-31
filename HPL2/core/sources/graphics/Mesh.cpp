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

#include "graphics/Mesh.h"

#include "graphics/SubMesh.h"
#include "resources/MaterialManager.h"
#include "resources/AnimationManager.h"
#include "graphics/VertexBuffer.h"
#include "graphics/Material.h"
#include "graphics/Skeleton.h"
#include "graphics/Animation.h"
#include "graphics/Bone.h"
#include "graphics/BoneState.h"

#include "math/Math.h"

#include "scene/Node3D.h"
#include "scene/MeshEntity.h"
#include "scene/LightSpot.h"
#include "scene/LightPoint.h"
#include "scene/World.h"
#include "scene/SoundEntity.h"
#include "scene/BillBoard.h"
#include "scene/Beam.h"
#include "scene/ParticleSystem.h"


namespace hpl {

  //////////////////////////////////////////////////////////////////////////
  // CONSTRUCTORS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------

  cMesh::cMesh(const tString& asName, const tWString& asFullPath, cMaterialManager* apMaterialManager, cAnimationManager* apAnimationManager)
      : iResourceBase(asName, asFullPath, 0) {
    mpMaterialManager  = apMaterialManager;
    mpAnimationManager = apAnimationManager;
    mpSkeleton         = NULL;

    mpRootNode = hplNew(cNode3D, ());
  }

  //-----------------------------------------------------------------------

  cMesh::~cMesh() {
    for (auto& mvSubMeshe : mvSubMeshes) {
      hplDelete(mvSubMeshe);
    }
    if (mpSkeleton) {
      hplDelete(mpSkeleton);
    }
    for (auto& mvAnimation : mvAnimations) {
      //mpAnimationManager->Destroy(mvAnimations[i]);
      hplDelete(mvAnimation);
    }
    if (mpRootNode) {
      hplDelete(mpRootNode);
    }
  }

  //-----------------------------------------------------------------------

  //////////////////////////////////////////////////////////////////////////
  // PUBLIC METHODS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------

  bool cMesh::CreateFromFile(const tString asFile) {
    return false;
  }

  //-----------------------------------------------------------------------

  cSubMesh* cMesh::CreateSubMesh(const tString& asName) {
    auto* pSubMesh = hplNew(cSubMesh, (asName, mpMaterialManager));

    pSubMesh->mpParent = this;

    mvSubMeshes.push_back(pSubMesh);
    m_mapSubMeshes.insert(tSubMeshMap::value_type(asName, pSubMesh));

    return pSubMesh;
  }

  //-----------------------------------------------------------------------

  cSubMesh* cMesh::GetSubMesh(unsigned int alIdx) {
    if (alIdx >= mvSubMeshes.size()) {
      return nullptr;
    }
    return mvSubMeshes[alIdx];
  }

  int cMesh::GetSubMeshIndex(const tString& asName) {
    for (size_t i = 0; i < mvSubMeshes.size(); ++i) {
      if (mvSubMeshes[i]->GetName() == asName) {
        return (int) i;
      }
    }
    return -1;
  }

  cSubMesh* cMesh::GetSubMeshName(const tString& asName) {
    auto it = m_mapSubMeshes.find(asName);
    if (it == m_mapSubMeshes.end()) {
      return nullptr;
    }
    return it->second;
  }

  int cMesh::GetSubMeshNum() {
    return (int) mvSubMeshes.size();
  }

  //-----------------------------------------------------------------------

  int cMesh::GetTriangleCount() {
    auto it = mvSubMeshes.begin();

    int lTriangleCount = 0;

    for (; it != mvSubMeshes.end(); ++it) {
      iVertexBuffer* pVB = (*it)->GetVertexBuffer();
      if (pVB) {
        lTriangleCount += (int) pVB->GetIndexNum() / 3;
      }
    }

    return lTriangleCount;
  }

  //-----------------------------------------------------------------------

  void cMesh::SetSkeleton(cSkeleton* apSkeleton) {
    mpSkeleton = apSkeleton;
  }

  cSkeleton* cMesh::GetSkeleton() {
    return mpSkeleton;
  }
  //-----------------------------------------------------------------------

  void cMesh::AddAnimation(cAnimation* apAnimation) {
    mvAnimations.push_back(apAnimation);

    tAnimationIndexMap::value_type value(apAnimation->GetName(), (int) mvAnimations.size() - 1);
    m_mapAnimIndices.insert(value);
  }

  cAnimation* cMesh::GetAnimation(int alIndex) {
    return mvAnimations[alIndex];
  }

  cAnimation* cMesh::GetAnimationFromName(const tString& asName) {
    int lIdx = GetAnimationIndex(asName);
    if (lIdx >= 0) {
      return mvAnimations[lIdx];
    } else {
      return nullptr;
    }
  }

  int cMesh::GetAnimationIndex(const tString& asName) {
    auto it = m_mapAnimIndices.find(asName);
    if (it != m_mapAnimIndices.end()) {
      return it->second;
    } else {
      return -1;
    }
  }

  void cMesh::ClearAnimations(bool abDeleteAll) {
    if (abDeleteAll) {
      for (auto& mvAnimation : mvAnimations) {
        //mpAnimationManager->Destroy(mvAnimations[i]);
        hplDelete(mvAnimation);
      }
    }

    mvAnimations.clear();
    m_mapAnimIndices.clear();
  }

  int cMesh::GetAnimationNum() {
    return (int) mvAnimations.size();
  }

  //-----------------------------------------------------------------------

  void cMesh::CompileBonesAndSubMeshes() {
    if (mpSkeleton == nullptr) {
      return;
    }

    ///////////////////////////////////////
    // Calculate the bounding radius for all bones
    // - this based on the greatest distance to an attached node.
    mvBoneBoundingRadii.resize(mpSkeleton->GetBoneNum(), 0);

    for (auto pSubMesh : mvSubMeshes) {
      ////////////////////////////
      //Get the variables
      iVertexBuffer* pVtxBuffer = pSubMesh->GetVertexBuffer();
      float*         pPosArray  = pVtxBuffer->GetFloatArray(eVertexBufferElement_Position);
      const int      lVtxStride = pVtxBuffer->GetElementNum(eVertexBufferElement_Position);

      ////////////////////////////
      //Iterate pairs and update the radii
      for (auto& pair : pSubMesh->mvVtxBonePairs) {
        float*    pPos = &pPosArray[pair.vtxIdx * lVtxStride];
        cVector3f vPos(pPos[0], pPos[1], pPos[2]);

        cBone* pBone = mpSkeleton->GetBoneByIndex(pair.boneIdx);

        float fDistSqr = cMath::Vector3DistSqr(vPos, pBone->GetWorldTransform().GetTranslation());
        float fRadius  = mvBoneBoundingRadii[pair.boneIdx];
        if (fDistSqr > fRadius * fRadius) {
          mvBoneBoundingRadii[pair.boneIdx] = sqrtf(fDistSqr);
        }
      }
    }
  }

  //-----------------------------------------------------------------------

  cNode3D* cMesh::GetRootNode() {
    return mpRootNode;
  }

  void cMesh::AddNode(cNode3D* apNode) {
    mvNodes.push_back(apNode);
  }

  int cMesh::GetNodeNum() {
    return (int) mvNodes.size();
  }

  cNode3D* cMesh::GetNodeByName(const tString& asName) {
    auto* pNode = (cNode3D*) STLFindByName(mvNodes, asName);
    return pNode;
  }

  cNode3D* cMesh::GetNode(int alIdx) {
    return mvNodes[alIdx];
  }

  //-----------------------------------------------------------------------


  //////////////////////////////////////////////////////////////////////////
  // PRIAVTE METHODS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------

  //-----------------------------------------------------------------------
} // namespace hpl
