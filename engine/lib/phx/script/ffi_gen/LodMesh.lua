-- LodMesh ---------------------------------------------------------------------
local Loader = {}

function Loader.declareType()
    ffi.cdef [[
        typedef struct LodMesh {} LodMesh;
    ]]

    return 1, 'LodMesh'
end

function Loader.defineType()
    local ffi = require('ffi')
    local libphx = require('libphx').lib
    local LodMesh

    do -- C Definitions
        ffi.cdef [[
            void     LodMesh_Free   (LodMesh*);
            LodMesh* LodMesh_Create ();
            LodMesh* LodMesh_Clone  (LodMesh const*);
            void     LodMesh_Add    (LodMesh*, Mesh* mesh, float distanceMin, float distanceMax);
            void     LodMesh_Draw   (LodMesh*, float distanceSquared);
            Mesh*    LodMesh_Get    (LodMesh*, float distanceSquared);
        ]]
    end

    do -- Global Symbol Table
        LodMesh = {
            Create = function(...)
                local instance = libphx.LodMesh_Create(...)
                return Core.ManagedObject(instance, libphx.LodMesh_Free)
            end,
        }

        if onDef_LodMesh then onDef_LodMesh(LodMesh, mt) end
        LodMesh = setmetatable(LodMesh, mt)
    end

    do -- Metatype for class instances
        local t  = ffi.typeof('LodMesh')
        local mt = {
            __index = {
                clone = function(...)
                    local instance = libphx.LodMesh_Clone(...)
                    return Core.ManagedObject(instance, libphx.LodMesh_Free)
                end,
                add   = libphx.LodMesh_Add,
                draw  = libphx.LodMesh_Draw,
                get   = function(...)
                    local instance = libphx.LodMesh_Get(...)
                    return Core.ManagedObject(instance, libphx.Mesh_Free)
                end,
            },
        }

        if onDef_LodMesh_t then onDef_LodMesh_t(t, mt) end
        LodMesh_t = ffi.metatype(t, mt)
    end

    return LodMesh
end

return Loader
