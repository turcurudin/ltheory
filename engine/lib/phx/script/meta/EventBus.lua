---@meta

---@class EventBus
EventBus = {}

---@param eventName string
---@param priority EventPriority
---@param frameStage FrameStage
---@param withFrameStageMessage boolean
function EventBus:register(eventName, priority, frameStage, withFrameStageMessage) end

---@param eventName string
function EventBus:unregister(eventName) end

---@param eventName string
---@param entityId integer
---@return integer
---@overload fun(self: table, eventName: string, ctxTable: table|nil, callbackFunc: function): integer
function EventBus:subscribe(eventName, entityId) end

---@param tunnelId integer
function EventBus:unsubscribe(tunnelId) end

---@param eventName string
---@param entityId integer
---@overload fun(self: table, eventName: string, ctxTable: table|nil)
function EventBus:send(eventName, entityId) end

---@return EventData
function EventBus:getNextEvent() end

function EventBus:printFrameStageMap() end
