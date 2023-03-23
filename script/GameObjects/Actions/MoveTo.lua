local Action = require('GameObjects.Action')
--local Bindings = require('States.ApplicationBindings')

local rng = RNG.FromTime()

local MoveTo = subclass(Action, function (self, target, range)
  self.target = target
  self.range = range
end)

function MoveTo:clone ()
  return MoveTo(self.target, self.range)
end

function MoveTo:getName ()
  local typename = Config:getObjectInfo("object_types", self.target:getType())
  return format("MoveTo %s '%s'", typename, self.target:getName())
end

local function getTargetPos (e, target)
  local tp = target:getPos()
  local tr = target:getRadius()
  local tu = target:getUp()
  local er = e:getRadius()
  return tp - tu:muls(1.25*tr + er)
end

function MoveTo:onUpdateActive (e, dt)
  -- Move to within the supplied range of the target object
  local tp = getTargetPos(e, self.target)

  -- Within range of the target object?
  if (e:getPos() - tp):length() <= self.range or (e == Config.game.currentShip and not Config.game.playerMoving) then
    -- MoveTo is complete, remove movement action from entity's Action queue
--printf("-> %s ended", e:getCurrentAction():getName())
    e:popAction()

    if e == Config.game.currentShip and Config.game.playerMoving then
      Config.game.playerMoving = false
    end

    return -- within range, so end flight
  end

  -- Use the "target" metaphor to store where this ship is moving to
  e:setTarget(self.target)

  if Config.debug.instantJobs then
    local p = e:getPos()
    local dp = tp - p
    e:setPos(p + dp:normalize():scale(rng:getUniform() * min(dp:length(), dt * Config.debug.jobSpeed)))
  else
    local tf = self.target:getForward()
    local tu = self.target:getUp()
    self:flyToward(e, tp, -tf, tu)
  end
end

return MoveTo
