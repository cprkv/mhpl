cmake_minimum_required(VERSION 3.4)
project(newton-core)

if(UNIX AND NOT APPLE)
    set(LINUX true)
endif()

if(LINUX)
    set(NEWTON_DEFINITIONS "-D_LINUX_VER -D_LINUX_VER_64")
elseif(APPLE)
    set(NEWTON_DEFINITIONS "-D_MAC_VER -D_NEWTON_USE_LIB -D_SCALAR_ARITHMETIC_ONLY") #TODO CHECK IT
elseif(WIN32)
    set(NEWTON_DEFINITIONS "-D_WIN_32_VER -D_NEWTON_USE_LIB")
endif()

set(NEWTON_DEFINITIONS "${NEWTON_DEFINITIONS} -D_NEWTON_STATIC_LIB")

set(SOURCES
    source/core/dg.cpp
    source/core/dgRef.cpp
    source/core/dgCRC.cpp
    source/core/dgTree.cpp
    source/core/dgNode.cpp
    source/core/dgTypes.cpp
    source/core/dgDebug.cpp
    source/core/dgMatrix.cpp
    source/core/dgGoogol.cpp
    source/core/dgMemory.cpp
    source/core/dgSphere.cpp
    source/core/dgRandom.cpp
    source/core/dgThreads.cpp
    source/core/dgPolyhedra.cpp
    source/core/dgQuaternion.cpp
    source/core/dgConvexHull3d.cpp
    source/core/dgConvexHull4d.cpp
    source/core/dgIntersections.cpp
    source/core/dgAABBPolygonSoup.cpp
    source/core/dgSmallDeterminant.cpp
    source/core/dgPolygonSoupBuilder.cpp
    source/core/dgPolyhedraMassProperties.cpp
    source/core/dgDelaunayTetrahedralization.cpp

    source/physics/dgWorld.cpp
    source/physics/dgBody.cpp
    source/physics/dgMeshEffect.cpp
    source/physics/dgMeshEffect2.cpp
    source/physics/dgMeshEffect3.cpp
    source/physics/dgMeshEffectSolidTree.cpp
    source/physics/dgCollisionConvex.cpp
    source/physics/dgBallConstraint.cpp
    source/physics/dgBilateralConstraint.cpp
    source/physics/dgCorkscrewConstraint.cpp
    source/physics/dgHingeConstraint.cpp
    source/physics/dgNarrowPhaseCollision.cpp
    source/physics/dgBroadPhaseCollision.cpp
    source/physics/dgMinkowskiConv.cpp
    source/physics/dgCollisionBox.cpp
    source/physics/dgCollisionCapsule.cpp
    source/physics/dgCollisionBVH.cpp
    source/physics/dgCollisionMesh.cpp
    source/physics/dgCollisionUserMesh.cpp
    source/physics/dgCollisionChamferCylinder.cpp
    source/physics/dgPointToCurveConstraint.cpp
    source/physics/dgCollisionCone.cpp
    source/physics/dgCollisionConvexHull.cpp
    source/physics/dgSlidingConstraint.cpp
    source/physics/dgCollisionConvexModifier.cpp
    source/physics/dgUniversalConstraint.cpp
    source/physics/dgCollision.cpp
    source/physics/dgUpVectorConstraint.cpp
    source/physics/dgCollisionCylinder.cpp
    source/physics/dgUserConstraint.cpp
    source/physics/dgCollisionEllipse.cpp
    source/physics/dgCollisionNull.cpp
    source/physics/dgCollisionScene.cpp
    source/physics/dgCollisionCompoundBreakable.cpp
    source/physics/dgCollisionHeightField.cpp
    source/physics/dgWorldDynamicUpdate.cpp
    source/physics/dgCollisionSphere.cpp
    source/physics/dgCollisionCompound.cpp
    source/physics/dgConnectorConstraint.cpp
    source/physics/dgConstraint.cpp
    source/physics/dgBodyMasterList.cpp
    source/physics/dgContact.cpp

    source/newton/Newton.cpp
    source/newton/NewtonClass.cpp
    )

add_library(newton-core STATIC ${SOURCES})
target_include_directories(newton-core PUBLIC
    source/core
    source/newton
    source/physics)
target_compile_definitions(newton-core PUBLIC ${NEWTON_DEFINITIONS})
