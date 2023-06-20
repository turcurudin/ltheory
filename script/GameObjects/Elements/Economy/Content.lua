-- This is a placeholder file for directly adding game content that would
-- typically be added in mod format

-- Items -----------------------------------------------------------------------

local Production   = require('Systems.Economy.Production')
local Item         = require('Systems.Economy.Item')

Item.T0            = {} -- nonsolid, energy
Item.T1            = {} -- nonsolid, information
Item.T2            = {} -- solid, raw, inanimate
Item.T3            = {} -- solid, raw, animate
Item.T4            = {} -- solid, processed, elemental
Item.T5            = {} -- solid, processed, constructed
Item.T6            = {} -- solid, processed, constructed, ship component
Item.T7            = {} -- solid, processed, constructed, ship hull
Item.T8            = {} -- solid, processed, constructed, station

Production.P0      = {} -- Powerplant
Production.P1      = {} -- Refinery
Production.P2      = {} -- Factory
Production.P3      = {} -- Drydock

--                         NAME                         UNIT MASS   ENERGY-DENSITY DISTRIBUTION %
Item.Energy        = Item("Energy Cell",                        1,            1.00,          1.00)
Item.Data          = Item("Data Cube",                          1,            2.10,          0.67)
Item.Information   = Item("Info Wafer",                         1,            3.82,          0.33)
Item.Silicates     = Item("Silicate Ore",                       4,            1.30,          0.30)
Item.Chondrites    = Item("Carboniferous Ore",                  6,            1.40,          0.19)
Item.WaterIce      = Item("Water Ice",                          5,            1.00,          0.13)
Item.BerylliumOre  = Item("Beryllium Ore",                      4,            1.45,          0.11)
Item.AluminumOre   = Item("Aluminum Ore",                       4,            1.60,          0.09)
Item.IronOre       = Item("Iron Ore",                           6,            1.68,          0.08)
Item.CopperOre     = Item("Copper Ore",                         7,            1.74,          0.06)
Item.ThoriumOre    = Item("Thorium Ore",                        7,            7.50,          0.04)
Item.Biomass       = Item("Biomass",                            4,            4.00,          0.00)
Item.Hydrogen      = Item("Hydrogen",                           1,            2.00,          1.00)
Item.Helium        = Item("Helium",                             1,            1.01,          1.00)
Item.Lithium       = Item("Lithium",                            1,            1.20,          1.00)
Item.Beryllium     = Item("Beryllium",                          2,            1.30,          1.00)
Item.Boron         = Item("Boron",                              2,            1.20,          1.00)
Item.Carbon        = Item("Carbon",                             2,            1.30,          1.00)
Item.Nitrogen      = Item("Nitrogen",                           1,            1.05,          1.00)
Item.Oxygen        = Item("Oxygen",                             1,            1.50,          1.00)
Item.Aluminum      = Item("Aluminum",                           3,            1.80,          1.00)
Item.Silicon       = Item("Silicon",                            2,            1.70,          1.00)
Item.Iron          = Item("Iron",                               5,            1.00,          1.00)
Item.Copper        = Item("Copper",                             5,            1.00,          1.00)
Item.Silver        = Item("Silver",                             5,            1.50,          1.00)
Item.Gold          = Item("Gold",                               6,            2.00,          1.00)
Item.Platinum      = Item("Platinum",                           8,            3.00,          1.00)
Item.Thorium       = Item("Thorium",                            9,           27.00,          1.00)
Item.Waste         = Item("Waste",                              1,            1.00,          1.00)
Item.WaterLiquid   = Item("Liquid Water",                       4,            1.00,          1.00)
Item.Steel         = Item("Steel",                              7,            1.00,          1.00)
Item.Isotopes      = Item("Radioactive Isotopes",              13,           50.00,          1.00)
Item.AnodeSludge   = Item("Anode Sludge",                       9,            1.00,          1.00)
Item.Petroleum     = Item("Petroleum",                          6,            3.50,          1.00)
Item.Plastic       = Item("Plastic",                            3,            1.90,          1.00)
Item.Glassiron     = Item("Glassiron",                         11,            1.00,          1.00)
Item.TranspAlum    = Item("Transparent Aluminum",               5,            1.70,          1.00)
Item.Infolytic     = Item("Infolytic Chip",                     3,            1.35,          1.00)
Item.WasteRad      = Item("Radioactive Waste",                 10,            2.50,          1.00)
Item.ShipComputer  = Item("Ship Computer",                    110,           22.00,          1.00)
Item.ShipSensor    = Item("Ship Sensor",                       80,           19.00,          1.00)
Item.EngineThrust  = Item("Ship Engine, Thruster",            600,           20.00,          1.00)
Item.EngineImpel   = Item("Ship Engine, Impeller",           1500,           18.00,          1.00)
Item.WeaponTPulse  = Item("Ship Weapon, Pulse Turret",        100,           10.00,          1.00)
Item.WeaponTBeam   = Item("Ship Weapon, Beam Turret",         120,           10.00,          1.00)
Item.WeaponTLaunch = Item("Ship Weapon, Launcher Turret",      90,            8.00,          1.00)
Item.WeaponBPulse  = Item("Ship Weapon, Pulse Bay",          1000,            9.00,          1.00)
Item.WeaponBBeam   = Item("Ship Weapon, Beam Bay",           1200,            9.00,          1.00)
Item.WeaponBCannon = Item("Ship Weapon, Cannon Bay",         2500,           10.00,          1.00)
Item.WeaponBLaunch = Item("Ship Weapon, Launcher Bay",        800,            7.00,          1.00)
Item.ShipSolo      = Item("Ship Hull, Solo",                15000,           50.00,          1.00)
Item.ShipSmall     = Item("Ship Hull, Small",               32000,           49.00,          1.00)
Item.ShipCompact   = Item("Ship Hull, Compact",             75000,           47.00,          1.00)
Item.ShipMedium    = Item("Ship Hull, Medium",             102500,           46.00,          1.00)
Item.ShipLarge     = Item("Ship Hull, Large",              237250,           45.00,          1.00)
Item.ShipVLarge    = Item("Ship Hull, Very Large",         518000,           42.00,          1.00)
Item.Station       = Item("Space Station, Small",         1550000,           40.00,          1.00)

insert(Item.T0, Item.Energy)
insert(Item.T1, Item.Data)
insert(Item.T1, Item.Information)
insert(Item.T2, Item.Silicates)
insert(Item.T2, Item.Chondrites)
insert(Item.T2, Item.WaterIce)
insert(Item.T2, Item.BerylliumOre)
insert(Item.T2, Item.AluminumOre)
insert(Item.T2, Item.IronOre)
insert(Item.T2, Item.CopperOre)
insert(Item.T2, Item.ThoriumOre)
insert(Item.T3, Item.Biomass)
insert(Item.T4, Item.Hydrogen)
insert(Item.T4, Item.Helium)
insert(Item.T4, Item.Lithium)
insert(Item.T4, Item.Beryllium)
insert(Item.T4, Item.Boron)
insert(Item.T4, Item.Carbon)
insert(Item.T4, Item.Nitrogen)
insert(Item.T4, Item.Oxygen)
insert(Item.T4, Item.Aluminum)
insert(Item.T4, Item.Silicon)
insert(Item.T4, Item.Iron)
insert(Item.T4, Item.Silver)
insert(Item.T4, Item.Platinum)
insert(Item.T4, Item.Gold)
insert(Item.T4, Item.Thorium)
insert(Item.T4, Item.Waste)
insert(Item.T4, Item.WaterLiquid)
insert(Item.T4, Item.Steel)
insert(Item.T4, Item.Isotopes)
insert(Item.T4, Item.AnodeSludge)
insert(Item.T5, Item.Petroleum)
insert(Item.T5, Item.Plastic)
insert(Item.T5, Item.Glassiron)
insert(Item.T5, Item.TranspAlum)
insert(Item.T5, Item.Infolytic)
insert(Item.T6, Item.ShipSensor)
insert(Item.T6, Item.ShipComputer)
insert(Item.T6, Item.EngineThrust)
insert(Item.T6, Item.EngineImpel)
insert(Item.T6, Item.WeaponTPulse)
insert(Item.T6, Item.WeaponTBeam)
insert(Item.T6, Item.WeaponTLaunch)
insert(Item.T6, Item.WeaponBPulse)
insert(Item.T6, Item.WeaponBBeam)
insert(Item.T6, Item.WeaponBCannon)
insert(Item.T6, Item.WeaponBLaunch)
insert(Item.T7, Item.ShipSolo)
insert(Item.T7, Item.ShipSmall)
insert(Item.T7, Item.ShipCompact)
insert(Item.T7, Item.ShipMedium)
insert(Item.T7, Item.ShipLarge)
insert(Item.T7, Item.ShipVLarge)
insert(Item.T8, Item.Station)

-- Production ------------------------------------------------------------------

local Production = require('Systems.Economy.Production')

Production.EnergySolar = Production("Solar Energy Array")
    :addOutput(Item.Energy, 10)
    :setDuration(8.0)
insert(Production.P0, Production.EnergySolar)

Production.EnergyNuclear = Production("Nuclear Reactor")
    :addInput(Item.Isotopes, 1)
    :addOutput(Item.Energy, 240)
    :addOutput(Item.WasteRad, 8)
    :setDuration(20.0)
insert(Production.P0, Production.EnergyNuclear)

Production.EnergyFusion = Production("Fusion Reactor")
    :addInput(Item.WaterLiquid, 320)
    :addOutput(Item.Energy, 1000)
    :addOutput(Item.WasteRad, 1)
    :setDuration(40.0)
insert(Production.P0, Production.EnergyFusion)

Production.Recycler = Production("Waste Recycler")
    :addInput(Item.Waste, 20)
    :addOutput(Item.Energy, 5)
    :setDuration(4.0)
insert(Production.P0, Production.Recycler)

Production.WaterMelter = Production("Water Melter")
    :addInput(Item.Energy, 2)
    :addInput(Item.WaterIce, 10)
    :addOutput(Item.WaterLiquid, 20)
    :setDuration(6.0)
insert(Production.P1, Production.WaterMelter)

Production.Silicon = Production("Silicon Refinery")
    :addInput(Item.Energy, 3)
    :addInput(Item.Silicates, 12)
    :addOutput(Item.Silicon, 1)
    :addOutput(Item.Waste, 6)
    :setDuration(12.0)
insert(Production.P1, Production.Silicon)

Production.Carbon = Production("Carbon Refinery")
    :addInput(Item.Energy, 3)
    :addInput(Item.Chondrites, 4)
    :addOutput(Item.Carbon, 1)
    :addOutput(Item.Waste, 3)
    :setDuration(13.0)
insert(Production.P1, Production.Carbon)

Production.Beryllium = Production("Beryllium Refinery")
    :addInput(Item.Energy, 8)
    :addInput(Item.BerylliumOre, 13)
    :addOutput(Item.Beryllium, 1)
    :addOutput(Item.Waste, 7)
    :setDuration(22.0)
insert(Production.P1, Production.Beryllium)

Production.Aluminum = Production("Aluminum Refinery")
    :addInput(Item.Energy, 6)
    :addInput(Item.AluminumOre, 8)
    :addOutput(Item.Aluminum, 1)
    :addOutput(Item.Waste, 5)
    :setDuration(15.0)
insert(Production.P1, Production.Aluminum)

Production.Iron = Production("Iron Refinery")
    :addInput(Item.Energy, 5)
    :addInput(Item.IronOre, 7)
    :addOutput(Item.Iron, 1)
    :addOutput(Item.Waste, 4)
    :setDuration(14.0)
insert(Production.P1, Production.Iron)

Production.Copper = Production("Copper Refinery")
    :addInput(Item.Energy, 7)
    :addInput(Item.CopperOre, 11)
    :addOutput(Item.Copper, 2)
    :addOutput(Item.AnodeSludge, 1)
    :addOutput(Item.Waste, 7)
    :setDuration(16.0)
insert(Production.P1, Production.Copper)

Production.Thorium = Production("Thorium Refinery")
    :addInput(Item.Energy, 13)
    :addInput(Item.ThoriumOre, 12)
    :addOutput(Item.Thorium, 1)
    :addOutput(Item.WasteRad, 2)
    :addOutput(Item.Waste, 5)
    :setDuration(20.0)
insert(Production.P1, Production.Thorium)

Production.Petroleum = Production("Petroleum Refinery")
    :addInput(Item.Energy, 6)
    :addInput(Item.Biomass, 22)
    :addOutput(Item.Petroleum, 1)
    :addOutput(Item.Waste, 15)
    :setDuration(14.0)
insert(Production.P1, Production.Petroleum)

Production.WaterSplitter = Production("Water Electrolyser")
    :addInput(Item.Energy, 3)
    :addInput(Item.WaterLiquid, 1)
    :addOutput(Item.Hydrogen, 2)
    :addOutput(Item.Oxygen, 1)
    :setDuration(5.0)
insert(Production.P2, Production.WaterSplitter)

Production.Steel = Production("Steel Foundry")
    :addInput(Item.Energy, 8)
    :addInput(Item.Iron, 8)
    :addInput(Item.Carbon, 2)
    :addOutput(Item.Steel, 1)
    :addOutput(Item.Waste, 4)
    :setDuration(13.0)
insert(Production.P2, Production.Steel)

Production.Silver = Production("Silver Recovery Plant")
    :addInput(Item.Energy, 12)
    :addInput(Item.AnodeSludge, 8)
    :addOutput(Item.Silver, 1)
    :addOutput(Item.Waste, 6)
    :setDuration(18.0)
insert(Production.P2, Production.Silver)

Production.Gold = Production("Gold Recovery Plant")
    :addInput(Item.Energy, 14)
    :addInput(Item.AnodeSludge, 11)
    :addOutput(Item.Gold, 1)
    :addOutput(Item.Waste, 7)
    :setDuration(21.0)
insert(Production.P2, Production.Gold)

Production.Platinum = Production("Platinum Recovery Plant")
    :addInput(Item.Energy, 20)
    :addInput(Item.AnodeSludge, 16)
    :addOutput(Item.Platinum, 1)
    :addOutput(Item.Waste, 9)
    :setDuration(25.0)
insert(Production.P2, Production.Platinum)

Production.Isotopes = Production("Isotope Factory")
    :addInput(Item.Energy, 5)
    :addInput(Item.Thorium, 12)
    :addOutput(Item.Isotopes, 1)
    :addOutput(Item.WasteRad, 7)
    :addOutput(Item.Waste, 3)
    :setDuration(11.0)
insert(Production.P2, Production.Isotopes)

Production.Plastic = Production("Plastics Factory")
    :addInput(Item.Energy, 11)
    :addInput(Item.Petroleum, 7)
    :addOutput(Item.Plastic, 1)
    :addOutput(Item.Waste, 8)
    :setDuration(9.0)
insert(Production.P2, Production.Plastic)

Production.Infolytic = Production("Infolytic Chip Factory")
    :addInput(Item.Energy, 10)
    :addInput(Item.Plastic, 8)
    :addInput(Item.Gold, 1)
    :addInput(Item.Silicon, 1)
    :addOutput(Item.Infolytic, 1)
    :addOutput(Item.Waste, 2)
    :setDuration(11)
insert(Production.P2, Production.Infolytic)

Production.TranspAluminum = Production("Transparent Aluminum Factory")
    :addInput(Item.Energy, 5)
    :addInput(Item.Silicon, 5)
    :addInput(Item.Aluminum, 3)
    :addOutput(Item.TranspAlum, 1)
    :addOutput(Item.Waste, 3)
    :setDuration(13.0)
insert(Production.P2, Production.TranspAluminum)

Production.Glassiron = Production("Glassiron Factory")
    :addInput(Item.Energy, 18)
    :addInput(Item.Silicon, 5)
    :addInput(Item.Iron, 3)
    :addInput(Item.Beryllium, 3)
    :addOutput(Item.Glassiron, 1)
    :addOutput(Item.Waste, 3)
    :setDuration(17.0)
insert(Production.P2, Production.Glassiron)

Production.ShipSensor = Production("Ship Sensor Factory")
    :addInput(Item.Energy, 210)
    :addInput(Item.Glassiron, 500)
    :addInput(Item.TranspAlum, 30)
    :addInput(Item.Steel, 75)
    :addInput(Item.Infolytic, 35)
    :addOutput(Item.ShipSensor, 1)
    :addOutput(Item.Waste, 55)
    :setDuration(450.0)
insert(Production.P2, Production.ShipSensor)

Production.ShipComputer = Production("Ship Computer Factory")
    :addInput(Item.Energy, 240)
    :addInput(Item.Glassiron, 420)
    :addInput(Item.TranspAlum, 28)
    :addInput(Item.Steel, 65)
    :addInput(Item.Silicon, 25)
    :addInput(Item.Infolytic, 150)
    :addOutput(Item.ShipComputer, 1)
    :addOutput(Item.Waste, 70)
    :setDuration(360.0)
insert(Production.P2, Production.ShipComputer)

Production.EngineThruster = Production("Thruster Engine Factory")
    :addInput(Item.Energy, 120)
    :addInput(Item.Glassiron, 1000)
    :addInput(Item.TranspAlum, 20)
    :addInput(Item.Steel, 170)
    :addInput(Item.Infolytic, 12)
    :addInput(Item.Thorium, 2)
    :addOutput(Item.EngineThrust, 1)
    :addOutput(Item.Waste, 55)
    :setDuration(900.0)
insert(Production.P2, Production.EngineThruster)

Production.TurretPulse = Production("Pulse Turret Weapon Factory")
    :addInput(Item.Energy, 180)
    :addInput(Item.Glassiron, 800)
    :addInput(Item.TranspAlum, 32)
    :addInput(Item.Steel, 210)
    :addInput(Item.Infolytic, 12)
    :addOutput(Item.WeaponTPulse, 1)
    :addOutput(Item.Waste, 40)
    :setDuration(790.0)
insert(Production.P2, Production.TurretPulse)

Production.ShipSolo = Production("Solo Ship Hull Drydock")
    :addInput(Item.Energy, 1500)
    :addInput(Item.Glassiron, 10000)
    :addInput(Item.TranspAlum, 900)
    :addInput(Item.Steel, 1850)
    :addInput(Item.Gold, 285)
    :addInput(Item.Platinum, 95)
    :addInput(Item.Infolytic, 165)
    :addInput(Item.Thorium, 35)
    :addInput(Item.ShipSensor, 1)
    :addInput(Item.ShipComputer, 1)
    :addInput(Item.EngineThrust, 1)
    :addOutput(Item.ShipSolo, 1)
    :addOutput(Item.Waste, 725)
    :setDuration(1800.0)
insert(Production.P3, Production.ShipSolo)

Production.Station = Production("Space Station Fabricator")
    :addInput(Item.Energy, 350000)
    :addInput(Item.Glassiron, 1000000)
    :addInput(Item.TranspAlum, 180000)
    :addInput(Item.Steel, 395000)
    :addInput(Item.Gold, 57000)
    :addInput(Item.Platinum, 9100)
    :addInput(Item.Infolytic, 8500)
    :addInput(Item.ShipSensor, 100)
    :addInput(Item.ShipComputer, 100)
    :addInput(Item.Thorium, 640)
    :addOutput(Item.Station, 1)
    :addOutput(Item.Waste, 9750)
    :setDuration(14400.0)
insert(Production.P3, Production.Station)

--------------------------------------------------------------------------------
