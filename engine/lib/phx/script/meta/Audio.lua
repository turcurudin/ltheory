---@meta

---@class Audio
Audio = {}

---@return Audio
function Audio.Create() end

---@param sound Sound
---@param initVolume number
---@param fadeMillis integer
---@return SoundInstance
function Audio:play(sound, initVolume, fadeMillis) end

---@param sound Sound
---@param initVolume number
---@param fadeMillis integer
---@param initPos Vec3f
---@param minDistance number
---@param maxDistance number
---@return SoundInstance
function Audio:play3D(sound, initVolume, fadeMillis, initPos, minDistance, maxDistance) end

---@param pos Vec3f
function Audio:setListenerPos(pos) end

---@return Vec3f
function Audio:listenerPos() end

---@param rot Quat
function Audio:setListenerRot(rot) end

---@return Quat
function Audio:listenerRot() end

---@return integer
function Audio:getLoadedCount() end

---@return integer
function Audio:getTotalCount() end
