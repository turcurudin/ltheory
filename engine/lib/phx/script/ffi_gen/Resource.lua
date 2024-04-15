-- Resource --------------------------------------------------------------------

---@class Resource
---@field Exists fun(ty: ResourceType, name: string): boolean
---@field GetPath fun(ty: ResourceType, name: string): string
---@field LoadBytes fun(ty: ResourceType, name: string): Bytes
---@field LoadString fun(ty: ResourceType, name: string): string

local Loader = {}

function Loader.declareType()
    return 0, 'Resource'
end

function Loader.defineType()
    local ffi = require('ffi')
    local libphx = require('libphx').lib
    local Resource

    do -- C Definitions
        ffi.cdef [[
            bool   Resource_Exists     (ResourceType ty, cstr name);
            cstr   Resource_GetPath    (ResourceType ty, cstr name);
            Bytes* Resource_LoadBytes  (ResourceType ty, cstr name);
            cstr   Resource_LoadString (ResourceType ty, cstr name);
        ]]
    end

    do -- Global Symbol Table
        Resource = {
            ---@param ty ResourceType
            ---@param name string
            ---@return boolean
            Exists     = libphx.Resource_Exists,
            ---@param ty ResourceType
            ---@param name string
            ---@return string
            GetPath    = libphx.Resource_GetPath,
            ---@param ty ResourceType
            ---@param name string
            ---@return Bytes
            LoadBytes  = function(...)
                local instance = libphx.Resource_LoadBytes(...)
                return Core.ManagedObject(instance, libphx.Bytes_Free)
            end,
            ---@param ty ResourceType
            ---@param name string
            ---@return string
            LoadString = libphx.Resource_LoadString,
        }

        if onDef_Resource then onDef_Resource(Resource, mt) end
        Resource = setmetatable(Resource, mt)
    end

    return Resource
end

return Loader
