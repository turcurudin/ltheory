local Action = require('GameObjects.Action')
local Player = require('GameObjects.Entities.Player')

local kJobIterations = 1000 -- how many randomly-chosen jobs the asset will consider before deciding

local Think = subclass(Action, function (self)
  self.timer = 0
  self.rng = RNG.FromTime()
end)

function Think:clone ()
  return Think()
end

function Think:getName ()
  return 'Think'
end

local function applyFlows (flows, mult)
  for _, flow in ipairs(flows) do
    flow.location:modFlow(flow.item, mult * flow.rate)
  end
end

--function Think:manageAsset (asset)
--  local root = asset:getRoot()
--  local bestPressure = asset.job and asset.job:getPressure(asset) or math.huge
--  local bestJob = asset.job
--  for i = 1, kJobIterations do
--    -- TODO : KnowsAbout check
--    local job = self.rng:choose(root:getEconomy().jobs)
--    if not job then break end
--
--    local pressure = job:getPressure(asset)
--    if pressure < bestPressure then
--      bestPressure = pressure
--      bestJob = job
----printf("[asset:%s] pressure = %s, job = %s", asset:getName(), pressure, job:getName())
--    end
--  end
--
--  if bestJob then
--    if asset.jobFlows then
--      applyFlows(asset.jobFlows, -1)
--      asset.jobFlows = nil
--    end
--
--    asset.job = bestJob
--    asset.jobFlows = bestJob:getFlows(asset)
--    applyFlows(asset.jobFlows, 1)
--
--    asset:pushAction(bestJob)
--  end
--end

if true then -- Use payout, not flow
  function Think:manageAsset (asset)
    local root = asset:getRoot()
    local bestPayout = 0
    local bestJob = nil

    -- Consider re-running last job
    if asset.job then
      local payout = asset.job:getPayout(asset)
      if payout > bestPayout then
        bestPayout = payout
        bestJob = asset.job
      end
    end

    -- Consider changing to a new job
    for i = 1, kJobIterations do
      -- TODO : KnowsAbout check (information economy + AI load reduction)
      local job = self.rng:choose(root:getEconomy().jobs)
      if not job then break end

      local payout = job:getPayout(asset)
      if payout > bestPayout then
        bestPayout = payout
        bestJob = job
      end
    end

    if bestJob then
      asset.job = bestJob
--printf("pushing action: %s", asset.job:getName())
      asset:pushAction(bestJob)
    end

    if asset:isIdle() and not asset:isShipDocked() then
      local system = asset.parent

      local stations = system:getStationsByDistance(asset)
      if #stations > 0 and stations[1] ~= nil then
        local station = stations[1].stationRef

printf("Asset '%s' has no more jobs available, docking at Station '%s'", asset:getName(), station:getName())
        asset:pushAction(Actions.DockAt(station))
      end
    end
  end
end

function Think:onUpdateActive (e, dt)
  Profiler.Begin('Action.Think')
  do -- Manage assets
    for asset in e:iterAssets() do
      if asset:getRoot():hasEconomy() and asset:isIdle() then
        self:manageAsset(asset)
      end
    end
  end

  self.timer = self.timer + dt
  do -- Capital expenditure
    if self.timer > 5 then
      --
    end
  end
  Profiler.End()
end

return Think
