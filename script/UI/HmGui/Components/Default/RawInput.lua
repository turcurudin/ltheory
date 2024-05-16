local RawInput = {}
RawInput.__index = RawInput

local meta = {
    __call = function(cls, ...)
        return cls:new(...)
    end
}

---@class UIRawInput: UIComponent
---@field widthInLayout number
---@field heightInLayout number
---@field render function renders the spacer

---@class UIRawInputConstructor
---@field widthInLayout number
---@field heightInLayout number
---@field fn function

---returns a spacer object
---@param args UIRawInputConstructor
---@return UIRawInput|nil
function RawInput:new(args)
    if not args then
        return
    end

    local newRawInput = {}
    newRawInput.state = UICore.ComponentState {
        widthInLayout = args.widthInLayout,
        heightInLayout = args.heightInLayout
    }

    newRawInput.render = function(self)
        args:fn()
        Gui:clearStyle() -- automatically call clearStyle() so styling doesn´t get applied to other components
    end

    return newRawInput
end

setmetatable(RawInput, meta)

-- Add to global UIComponent table
---@type UIComponentSpacerConstructor
UIComponent.RawInput = RawInput

return RawInput