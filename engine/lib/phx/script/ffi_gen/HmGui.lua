-- HmGui -----------------------------------------------------------------------
local Loader = {}

function Loader.declareType()
    ffi.cdef [[
        typedef struct HmGui {} HmGui;
    ]]

    return 1, 'HmGui'
end

function Loader.defineType()
    local ffi = require('ffi')
    local libphx = require('libphx').lib
    local HmGui

    do -- C Definitions
        ffi.cdef [[
            void  HmGui_BeginGui         (HmGui*, float sx, float sy, Input const* input);
            void  HmGui_EndGui           (HmGui*, Input const* input);
            void  HmGui_Draw             (HmGui*);
            void  HmGui_BeginGroupX      (HmGui*);
            void  HmGui_BeginGroupY      (HmGui*);
            void  HmGui_BeginGroupStack  (HmGui*);
            void  HmGui_EndGroup         (HmGui*);
            void  HmGui_BeginScroll      (HmGui*, float maxSize);
            void  HmGui_EndScroll        (HmGui*, Input const* input);
            void  HmGui_BeginWindow      (HmGui*, cstr title, Input const* input);
            void  HmGui_EndWindow        (HmGui*);
            bool  HmGui_Button           (HmGui*, cstr label);
            bool  HmGui_Checkbox         (HmGui*, cstr label, bool value);
            float HmGui_Slider           (HmGui*, float lower, float upper, float value);
            void  HmGui_Image            (HmGui*, Tex2D* image);
            void  HmGui_Rect             (HmGui*, float sx, float sy, float r, float g, float b, float a);
            void  HmGui_Text             (HmGui*, cstr text);
            void  HmGui_TextColored      (HmGui*, cstr text, float r, float g, float b, float a);
            void  HmGui_TextEx           (HmGui*, Font* font, cstr text, float r, float g, float b, float a);
            void  HmGui_SetAlign         (HmGui*, float ax, float ay);
            void  HmGui_SetPadding       (HmGui*, float px, float py);
            void  HmGui_SetPaddingEx     (HmGui*, float left, float top, float right, float bottom);
            void  HmGui_SetPaddingLeft   (HmGui*, float padding);
            void  HmGui_SetPaddingTop    (HmGui*, float padding);
            void  HmGui_SetPaddingRight  (HmGui*, float padding);
            void  HmGui_SetPaddingBottom (HmGui*, float padding);
            void  HmGui_SetSpacing       (HmGui*, float spacing);
            void  HmGui_SetStretch       (HmGui*, float x, float y);
            bool  HmGui_GroupHasFocus    (HmGui*, FocusType ty);
            void  HmGui_PushStyle        (HmGui*);
            void  HmGui_PushFont         (HmGui*, Font* font);
            void  HmGui_PushTextColor    (HmGui*, float r, float g, float b, float a);
            void  HmGui_PopStyle         (HmGui*, int depth);
        ]]
    end

    do -- Global Symbol Table
        HmGui = {
            BeginGui         = libphx.HmGui_BeginGui,
            EndGui           = libphx.HmGui_EndGui,
            Draw             = libphx.HmGui_Draw,
            BeginGroupX      = libphx.HmGui_BeginGroupX,
            BeginGroupY      = libphx.HmGui_BeginGroupY,
            BeginGroupStack  = libphx.HmGui_BeginGroupStack,
            EndGroup         = libphx.HmGui_EndGroup,
            BeginScroll      = libphx.HmGui_BeginScroll,
            EndScroll        = libphx.HmGui_EndScroll,
            BeginWindow      = libphx.HmGui_BeginWindow,
            EndWindow        = libphx.HmGui_EndWindow,
            Button           = libphx.HmGui_Button,
            Checkbox         = libphx.HmGui_Checkbox,
            Slider           = libphx.HmGui_Slider,
            Image            = libphx.HmGui_Image,
            Rect             = libphx.HmGui_Rect,
            Text             = libphx.HmGui_Text,
            TextColored      = libphx.HmGui_TextColored,
            TextEx           = libphx.HmGui_TextEx,
            SetAlign         = libphx.HmGui_SetAlign,
            SetPadding       = libphx.HmGui_SetPadding,
            SetPaddingEx     = libphx.HmGui_SetPaddingEx,
            SetPaddingLeft   = libphx.HmGui_SetPaddingLeft,
            SetPaddingTop    = libphx.HmGui_SetPaddingTop,
            SetPaddingRight  = libphx.HmGui_SetPaddingRight,
            SetPaddingBottom = libphx.HmGui_SetPaddingBottom,
            SetSpacing       = libphx.HmGui_SetSpacing,
            SetStretch       = libphx.HmGui_SetStretch,
            GroupHasFocus    = libphx.HmGui_GroupHasFocus,
            PushStyle        = libphx.HmGui_PushStyle,
            PushFont         = libphx.HmGui_PushFont,
            PushTextColor    = libphx.HmGui_PushTextColor,
            PopStyle         = libphx.HmGui_PopStyle,
        }

        if onDef_HmGui then onDef_HmGui(HmGui, mt) end
        HmGui = setmetatable(HmGui, mt)
    end

    do -- Metatype for class instances
        local t  = ffi.typeof('HmGui')
        local mt = {
            __index = {
                beginGui         = libphx.HmGui_BeginGui,
                endGui           = libphx.HmGui_EndGui,
                draw             = libphx.HmGui_Draw,
                beginGroupX      = libphx.HmGui_BeginGroupX,
                beginGroupY      = libphx.HmGui_BeginGroupY,
                beginGroupStack  = libphx.HmGui_BeginGroupStack,
                endGroup         = libphx.HmGui_EndGroup,
                beginScroll      = libphx.HmGui_BeginScroll,
                endScroll        = libphx.HmGui_EndScroll,
                beginWindow      = libphx.HmGui_BeginWindow,
                endWindow        = libphx.HmGui_EndWindow,
                button           = libphx.HmGui_Button,
                checkbox         = libphx.HmGui_Checkbox,
                slider           = libphx.HmGui_Slider,
                image            = libphx.HmGui_Image,
                rect             = libphx.HmGui_Rect,
                text             = libphx.HmGui_Text,
                textColored      = libphx.HmGui_TextColored,
                textEx           = libphx.HmGui_TextEx,
                setAlign         = libphx.HmGui_SetAlign,
                setPadding       = libphx.HmGui_SetPadding,
                setPaddingEx     = libphx.HmGui_SetPaddingEx,
                setPaddingLeft   = libphx.HmGui_SetPaddingLeft,
                setPaddingTop    = libphx.HmGui_SetPaddingTop,
                setPaddingRight  = libphx.HmGui_SetPaddingRight,
                setPaddingBottom = libphx.HmGui_SetPaddingBottom,
                setSpacing       = libphx.HmGui_SetSpacing,
                setStretch       = libphx.HmGui_SetStretch,
                groupHasFocus    = libphx.HmGui_GroupHasFocus,
                pushStyle        = libphx.HmGui_PushStyle,
                pushFont         = libphx.HmGui_PushFont,
                pushTextColor    = libphx.HmGui_PushTextColor,
                popStyle         = libphx.HmGui_PopStyle,
            },
        }

        if onDef_HmGui_t then onDef_HmGui_t(t, mt) end
        HmGui_t = ffi.metatype(t, mt)
    end

    return HmGui
end

return Loader
