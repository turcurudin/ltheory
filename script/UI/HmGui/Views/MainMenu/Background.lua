---@type UIView
local BackgroundView = UICore.View {
    name = "Background"
}

---@type UIRouter
local UIRouter = require("UI.HmGui.UICore.UIRouter")

local lastMousePos = Vec2f(0, 0)
local lastMoved = TimeStamp.Now()
local backButtonVisible = true

function BackgroundView:onInput()
    local mousePos = InputInstance:mouse():position()

    if mousePos.x ~= lastMousePos.x and mousePos.y ~= lastMousePos.y then
        lastMoved = TimeStamp.Now()
        lastMousePos = mousePos
        backButtonVisible = true
        InputInstance:setCursorVisible(true)
    end

    if lastMoved:getElapsed() >= 5 then
        backButtonVisible = false
        InputInstance:setCursorVisible(false)
    end
end

function BackgroundView:onUpdate(dt) end

local function getButtonWidth()
    return GameState.render.resX / 1600 * 200
end

local function getButtonHeight()
    return GameState.render.resY / 900 * 40
end

local function getBackButtonVisible()
    return backButtonVisible
end

local function switchToMainScreen()
    UIRouter:getCurrentPage():setView("Main")
end

local container = UIComponent.Container {
    align = { AlignHorizontal.Stretch, AlignVertical.Stretch },
    padding = { 50, 50 },
    margin = { 0, 0 },
    stackDirection = Enums.UI.StackDirection.Horizontal,
    contents = {
        UIComponent.Container {
            align = { AlignHorizontal.Center, AlignVertical.Center },
            padding = { 50, 10 },
            margin = { 0, 0 },
            stackDirection = Enums.UI.StackDirection.Vertical,
            contents = {
                UIComponent.Button {
                    visible = getBackButtonVisible,
                    title = "Back",
                    width = getButtonWidth,
                    height = getButtonHeight,
                    align = { AlignHorizontal.Center, AlignVertical.Center },
                    callback = switchToMainScreen
                },
            }
        }
    }
}

BackgroundView:addContent(container)

return BackgroundView
