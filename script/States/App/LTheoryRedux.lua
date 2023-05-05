--** REQUIRES **--
local Player = require('GameObjects.Entities.Player')
local System = require('GameObjects.Entities.Test.System')
local DebugControl = require('Systems.Controls.Controls.DebugControl')
local Bindings = require('States.ApplicationBindings')
local ShipBindings = require('Systems.Controls.Bindings.ShipBindings')
local Actions = requireAll('GameObjects.Actions')
local SocketType = require('GameObjects.Entities.Ship.SocketType')
local InitFiles = require('Systems.Files.InitFiles')
local MainMenu = require('Systems.Menus.MainMenu')
local MusicPlayer = require('Systems.SFX.MusicPlayer')
local Universe = require('Systems.Universe.Universe')

LTheoryRedux = require('States.Application')

--** LOCAL VARIABLES **--
local newSound = nil
local newSeed = 0ULL
local newShip = nil
local bNewSSystem = false
local bShowSystemMap = false
local bSMapAdded = false
local smap = nil

local rng = RNG.FromTime()

--** MAIN CODE **--
function LTheoryRedux:onInit ()
  --* Value initializations *--
  self.logo = Tex2D.Load("./res/images/LTR_logo2.png") -- load the LTR logo

  DebugControl.ltheory = self

  -- Read user-defined values and update game variables
  InitFiles:readUserInits()

  --* Audio initializations *--
  Audio.Init()
  Audio.Set3DSettings(0.0, 10, 2);

  if Config.audio.pulseFire then Sound.SetVolume(Config.audio.pulseFire, Config.audio.soundMax) end

  -- Open Main Menu
  MusicPlayer:Init()
  MainMenu:Open()

  --* Game initializations *--
  self.window:setSize(Config.render.startingHorz, Config.render.startingVert)
  Window.SetPosition(self.window, WindowPos.Centered, WindowPos.Centered)
  if Config.render.fullscreen then
    self.window:toggleFullscreen()
  end

  -- Set the default game control cursor
  self.window:setCursor(Config.ui.cursor, Config.ui.cursorX, Config.ui.cursorY)

  self.player = Entities.Player(Config.game.humanPlayerName)
  Config.game.humanPlayer = self.player
  self:generate()
end

function LTheoryRedux:toggleSound ()
  Config.audio.bSoundOn = not Config.audio.bSoundOn

  if Config.audio.bSoundOn then
    MusicPlayer:SetVolume(1)
  else
    MusicPlayer:SetVolume(0)
  end
end

function LTheoryRedux:SoundOn ()
  Config.audio.bSoundOn = true
  MusicPlayer:SetVolume(1)
end

function LTheoryRedux:SoundOff ()
  Config.audio.bSoundOn = false
  MusicPlayer:SetVolume(0)
end

function LTheoryRedux:onInput ()
  self.canvas:input()
end

function LTheoryRedux:onDraw ()
  -- Check to see whether to draw the System Map or the game world onto the canvas
  if bShowSystemMap then
    if not bSMapAdded then
      self.canvas:remove(self.gameView)
      self.canvas:add(smap)
      bSMapAdded = true
      Input.SetMouseVisible(true)
      print("Draw System View")
    end
  else
    if smap ~= nil then
      self.canvas:remove(smap)
      self.canvas:add(self.gameView)
      bSMapAdded = false
      smap = nil
      Input.SetMouseVisible(false)
      print("Draw Game View")
    end
  end

  self.canvas:draw(self.resX, self.resY)

  HmGui.Draw() -- draw controls
end

function LTheoryRedux:onUpdate (dt)
  -- Routes
  self.player:getRoot():update(dt)
  self.canvas:update(dt)
  MainMenu:OnUpdate(dt)
  MusicPlayer:OnUpdate(dt)
  UniverseEconomy:OnUpdate(dt)

  -- TODO: Confirm whether this is still needed
  local playerShip = self.player
  if playerShip ~= nil then
    playerShip = Config.game.currentShip
  end

  if Bindings.All:get() == 1 then
    -- Take down splash text if pretty much any key is pressed
    if MainMenu.currentMode == Enums.MenuMode.Splashscreen then
      MainMenu:SetBackgroundMode(false)
      MainMenu:SetMenuMode(Enums.MenuMode.MainMenu) -- show Main Menu
    end
    MainMenu:ActionRegistered()
  end

  if not MainMenu.enabled and MainMenu.currentMode == Enums.MenuMode.MainMenu then
    MainMenu:Open()
  elseif MainMenu.enabled and MainMenu.currentMode == Enums.MenuMode.Dialog then
    MainMenu:Close(true)
  end

  -- Manage game control screens
  if MainMenu.currentMode ~= Enums.MenuMode.Splashscreen and Input.GetPressed(Bindings.Escape) then
    MainMenu:SetBackgroundMode(false)
    if Config.getGameMode() == 1 then
      MainMenu:SetMenuMode(Enums.MenuMode.MainMenu) -- show Main Menu
    else
      -- First time here, menuMode should be 0 (just starting game), so don't pop up the Flight Mode dialog box
      -- After that, in active Flight Mode, do pop up the Flight Mode dialog box when the player presses ESC
      if MainMenu.currentMode == Enums.MenuMode.Splashscreen then
        Config.game.flightModeButInactive = false
        MainMenu:SetMenuMode(Enums.MenuMode.Dialog) -- show Flight Mode dialog
      elseif MainMenu.currentMode == Enums.MenuMode.Dialog and not MainMenu.seedDialogDisplayed then
        Config.game.flightModeButInactive = not Config.game.flightModeButInactive
        Input.SetMouseVisible(Config.game.flightModeButInactive)

        if Config.game.flightModeButInactive then
          Config.game.gamePaused = true
        else
          Config.game.gamePaused = false
        end
      end
    end
  end

  -- If player pressed the "System Map" key in Flight Mode, toggle the system map's visibility
  if Input.GetPressed(Bindings.SystemMap) and MainMenu.currentMode == Enums.MenuMode.Dialog then
    bShowSystemMap = not bShowSystemMap
    if smap == nil then
      smap = Systems.CommandView.SystemMap(self.system)
    end
  end

  -- If in flight mode, engage autopilot
  if Input.GetPressed(Bindings.AutoNav) and MainMenu.currentMode == Enums.MenuMode.Dialog then
    if playerShip ~= nil then
      local target = playerShip:getTarget()
      if target == nil then target = self.focus end
      if not playerShip:isDestroyed() and playerShip:isShipDocked() == nil and target ~= nil and target ~= playerShip then
        if playerShip:getCurrentAction() == nil or not string.find(playerShip:getCurrentAction():getName(),"MoveTo") then
          -- Move undestroyed, undocked player ship to area of selected target
          local autodistance = Config.game.autonavRanges[target:getType()]
          Config.game.autonavTimestamp = Config.getCurrentTimestamp()
          Config.game.playerMoving = true -- must be set to true before pushing the MoveTo action
          playerShip:pushAction(Actions.MoveTo(target, autodistance))
        end
      end
    end
  end

  -- Disengage autopilot (require a 1-second delay, otherwise keypress turns autopilot on then off instantly)
  if Config.game.playerMoving then
    if Input.GetPressed(Bindings.AutoNav) and Config.getCurrentTimestamp() - Config.game.autonavTimestamp > 1 then
      Config.game.playerMoving = false
    end
  end

  -- If player pressed the "ToggleLights" key in Flight Mode, toggle dynamic lighting on/off
  -- NOTE: Performance is OK for just the player's ship, but adding many lit ships & pulses tanks performance
  if Input.GetPressed(Bindings.ToggleLights) and MainMenu.currentMode == Enums.MenuMode.Dialog then
    Config.render.thrusterLights = not Config.render.thrusterLights
    Config.render.pulseLights    = not Config.render.pulseLights
  end

  -- Decide which game controls screens (if any) to display on top of the canvas
  HmGui.Begin(self.resX, self.resY)

  if MainMenu.currentMode == Enums.MenuMode.Splashscreen then
    LTheoryRedux:showGameLogo()
  elseif MainMenu.currentMode == Enums.MenuMode.MainMenu then
    if not MainMenu.inBackgroundMode then
      if MainMenu.seedDialogDisplayed then
        MainMenu:ShowSeedDialog()
      elseif MainMenu.settingsScreenDisplayed then
        MainMenu:ShowSettingsScreen()
      else
        MainMenu:ShowGui()
      end
    end
  elseif MainMenu.currentMode == Enums.MenuMode.Dialog then
    if Config.game.flightModeButInactive then
      MainMenu:ShowFlightDialog()
    elseif MainMenu.seedDialogDisplayed then
      MainMenu:ShowSeedDialog()
    elseif MainMenu.settingsScreenDisplayed then
      MainMenu:ShowSettingsScreen()
    end
  end
  HmGui.End()

  -- If player pressed the "new background" key and we're in startup mode, generate a new star system for a background
  if Input.GetPressed(Bindings.NewBackground) and MainMenu.currentMode == Enums.MenuMode.MainMenu then
    LTheoryRedux:seedStarsystem(Enums.MenuMode.MainMenu)
  end

  -- If player pressed the "toggle audio" key, turn it off if it's on or on if it's off
  if Input.GetPressed(Bindings.ToggleSound) then
    LTheoryRedux:toggleSound()
  end
end

function LTheoryRedux:generateNewSeed ()
  self.seed = rng:get64()
end

function LTheoryRedux:generate ()
  Config.setGameMode(1) -- start off in Startup Mode

  -- Use random seed for new background star system, and stay in "display game logo" startup mode
  LTheoryRedux:seedStarsystem(Enums.MenuMode.Splashscreen)
end

function LTheoryRedux:seedStarsystem (menuMode)
  self.seed = rng:get64()

  LTheoryRedux:createStarSystem()

  MainMenu:SetMenuMode(menuMode)
end

function LTheoryRedux:createStarSystem ()
  if self.system then self.system:delete() end

  print("------------------------")
  if Config.getGameMode() == 1 then
    -- Use custom system generation sizes for a nice background star system
    Config.gen.scaleSystem    = Config.gen.scaleSystemBack
    Config.gen.scalePlanet    = Config.gen.scalePlanetBack
    Config.gen.scalePlanetMod = Config.gen.scalePlanetModBack
    Config.render.zNear       = Config.gen.zNearBack
    Config.render.zFar        = Config.gen.zFarBack
  else
    -- Use the "real" system generation sizes for a gameplay star system
    Config.gen.scaleSystem    = Config.gen.scaleSystemReal
    Config.gen.scalePlanet    = Config.gen.scalePlanetReal
    Config.gen.scalePlanetMod = Config.gen.scalePlanetModReal
    Config.render.zNear       = Config.gen.zNearReal
    Config.render.zFar        = Config.gen.zFarReal
  end

  -- Spawn a new star system
  self.system = System(self.seed)
  Config.game.currentSystem = self.system -- remember the player's current star system

  do
    if Config.getGameMode() == 1 then
      -- Background Mode
      -- Generate a new star system with nebulae/dust, a planet, an asteroid field,
      --   a space station, and an invisible rotating ship
      newShip = self.system:spawnBackground() -- spawn an invisible ship
      LTheoryRedux:insertShip(newShip)

      -- Add a planet
      for i = 1, 1 do
        local planet = self.system:spawnPlanet(false) -- no planetary asteroid belt
        local ppos = planet:getPos()
        ppos.x = ppos.x * 2
        ppos.y = ppos.y * 2
        planet:setPos(ppos) -- move planet away from origin for background
      end

      -- Add an asteroid field
      -- Must add BEFORE space stations
      for i = 1, rng:getInt(0, 1) do -- 50/50 chance of having asteroids
        self.system:spawnAsteroidField(-1, true) -- -1 is a special case meaning background
      end

      -- Add a space station
      local station = self.system:spawnStation(Config.game.humanPlayer, nil)
    else
      -- Flight Mode

      -- Reset variables used between star systems
      Config.game.gamePaused   = false
      Config.game.panelActive  = false
      Config.game.playerMoving = false
      Config.game.weaponGroup  = 1

      -- Generate a new star system with nebulae/dust, a planet, an asteroid field,
      --   a space station, a visible pilotable ship, and possibly some NPC ships
      local afield = nil

      -- Add planets
      local planet = nil -- remember the last planet created (TODO: remember ALL the planets)
      for i = 1, Config.gen.nPlanets do
        planet = self.system:spawnPlanet(false)
      end

      -- Add asteroid fields
      -- Must add BEFORE space stations
      for i = 1, Config.gen.nFields do
        afield = self.system:spawnAsteroidField(Config.gen.nAsteroids, false)
        printf("Added %s asteroids to %s", Config.gen.nAsteroids, afield:getName())
      end

      -- Add the player's ship
      newShip = self.system:spawnShip(Config.game.humanPlayer)
      newShip:setName(Config.game.humanPlayerShipName)
      newShip:setHealth(500, 500, 10) -- make the player's ship healthier than the default NPC ship

      LTheoryRedux:insertShip(newShip)

      Config.game.currentShip = newShip

      -- Set our ship's starting location within the extent of a random asteroid field
      self.system:place(newShip)
printf("Added our ship, the '%s', at pos %s", newShip:getName(), newShip:getPos())

      -- Add System to the UniverseEconomy
      Universe:AddStarSystem(self.system)
    end
  end
  -- Insert the game view into the application canvas to make it visible
  self.gameView = Systems.Overlay.GameView(self.player)
  self.canvas = UI.Canvas()
  self.canvas
    :add(self.gameView
      :add(Systems.Controls.Controls.MasterControl(self.gameView, self.player))
    )

  -- Temporary until game states are properly introduced
  if Config.getGameMode() == 2 then
    MusicPlayer:PlayAmbient()
  end

  -- Set the initial mouse position when Flight mode begins to the center of the game window
  self.window:setWindowGrab(true)
  Input.SetMousePosition(self.resX / 2, self.resY / 2)
  self.window:setWindowGrab(false)
end

function LTheoryRedux:insertShip(ourShip)
  -- Insert ship into this star system
  ourShip:setPos(Config.gen.origin)
  ourShip:setFriction(0)
  ourShip:setSleepThreshold(0, 0)
  ourShip:setOwner(self.player)
  self.system:addChild(ourShip)
  self.player:setControlling(ourShip)
end

function LTheoryRedux:showGameLogo ()
  -- Draw the LTR game logo on top of the background star system
  local scaleFactor = ((self.resX * self.resY) / (1600 * 900)) ^ 0.5
  local scaleFactorX = self.resX / 1600
  local scaleFactorY = self.resY /  900
  HmGui.Image(self.logo) -- draw the LTR logo on top of the canvas
  HmGui.SetStretch(0.76 * scaleFactor / scaleFactorX, 0.243 * scaleFactor / scaleFactorY) -- scale logo (width, height)
  HmGui.SetAlign(0.5, 0.5) -- align logo
end

function LTheoryRedux:exitGame ()
  -- Shut down game and exit
  MusicPlayer:SetVolume(0)

  -- Write player-specific game variables to preserve them across gameplay sessions
  InitFiles:writeUserInits()

  LTheoryRedux:quit()
end

--** SUPPORT FUNCTIONS **--
function LTheoryRedux:freezeTurrets ()
  -- When taking down a dialog, Turret:updateTurret sees the button click input and thinks it means "Fire"
  -- So this routine adds a very brief cooldown to the player ship's turrets
  if Config.game.currentShip then
    for turret in Config.game.currentShip:iterSocketsByType(SocketType.Turret) do
      turret:addCooldown(2.0)
    end
  end
end

function LTheoryRedux:sleep (sec)
  os.execute(package.config:sub(1,1) == "/" and "sleep " .. sec or "timeout " .. sec )
end

return LTheoryRedux
