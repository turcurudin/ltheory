-- MouseState ------------------------------------------------------------------
local ffi = require('ffi')
local libphx = require('ffi.libphx').lib
local MouseState

do -- C Definitions
    ffi.cdef [[
        typedef struct MouseState {} MouseState;

        float MouseState_Value      (MouseState const*, MouseControl control);
        bool  MouseState_IsPressed  (MouseState const*, MouseControl control);
        bool  MouseState_IsDown     (MouseState const*, MouseControl control);
        bool  MouseState_IsReleased (MouseState const*, MouseControl control);
    ]]
end

do -- Global Symbol Table
    MouseState = {
        Value      = libphx.MouseState_Value,
        IsPressed  = libphx.MouseState_IsPressed,
        IsDown     = libphx.MouseState_IsDown,
        IsReleased = libphx.MouseState_IsReleased,
    }

    if onDef_MouseState then onDef_MouseState(MouseState, mt) end
    MouseState = setmetatable(MouseState, mt)
end

do -- Metatype for class instances
    local t  = ffi.typeof('MouseState')
    local mt = {
        __index = {
            value      = libphx.MouseState_Value,
            isPressed  = libphx.MouseState_IsPressed,
            isDown     = libphx.MouseState_IsDown,
            isReleased = libphx.MouseState_IsReleased,
        },
    }

    if onDef_MouseState_t then onDef_MouseState_t(t, mt) end
    MouseState_t = ffi.metatype(t, mt)
end

return MouseState
