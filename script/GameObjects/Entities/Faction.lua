local Entity = require('GameObjects.Entity')

local Faction = subclass(Entity, function(self, def)
  self:addGuid()
  self:setName(def.name)
  self:setType(Enums.EntityType.Faction)
  self:setFactionType(def.type)

  if def.owner then
    self:setOwner(def.owner)
    def.owner:setFaction(self)
  end
end)

return Faction
