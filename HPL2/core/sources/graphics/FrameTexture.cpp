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

#include "graphics/FrameTexture.h"
#include "graphics/Texture.h"
#include "graphics/FrameSubImage.h"
#include "resources/ImageManager.h"

namespace hpl {

  //////////////////////////////////////////////////////////////////////////
  // CONSTRUCTORS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------

  cFrameTexture::cFrameTexture(iTexture* apTex, int alHandle, cImageManager* apImageManager, bool abIsCustom)
      : iFrameBase() {
    mpTexture      = apTex;
    mlHandle       = alHandle;
    mbIsCustom     = abIsCustom;
    mpImageManager = apImageManager;
  }

  cFrameTexture::~cFrameTexture() {
    if (mpTexture) {
      hplDelete(mpTexture);
    }
    mpTexture = nullptr;
  }

  //-----------------------------------------------------------------------

  //////////////////////////////////////////////////////////////////////////
  // PUBLIC METHODS
  //////////////////////////////////////////////////////////////////////////

  //-----------------------------------------------------------------------

  iTexture* cFrameTexture::GetTexture() {
    return mpTexture;
  }

  //-----------------------------------------------------------------------

  cFrameSubImage* cFrameTexture::CreateCustomImage(const cVector2l& avPixelPos, const cVector2l& avPixelSize) {
    if (!mbIsCustom) {
      return nullptr;
    }

    mlPicCount++;

    const cVector3l& vSourceSize = mpTexture->GetSize();
    cVector2f        vDestPos    = cVector2f(
        static_cast<float>(avPixelPos.x) / static_cast<float>(vSourceSize.x),
        static_cast<float>(avPixelPos.y) / static_cast<float>(vSourceSize.y));
    cVector2f vDestSize = cVector2f(
        static_cast<float>(avPixelSize.x) / static_cast<float>(vSourceSize.x),
        static_cast<float>(avPixelSize.y) / static_cast<float>(vSourceSize.y));

    auto* pImage = hplNew(cFrameSubImage, ("", _W(""), this, nullptr,
                                           cRect2l(avPixelPos, avPixelSize),
                                           cVector2l(vSourceSize.x, vSourceSize.y),
                                           mlHandle, nullptr));

    pImage->IncUserCount();

    return pImage;
  }

  //-----------------------------------------------------------------------
} // namespace hpl
