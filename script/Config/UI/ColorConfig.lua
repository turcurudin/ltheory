Config.ui.color = {
    --                          R     G     B     A
    accent            = Color(1.00, 0.00, 0.30, 1.0),
    focused           = Color(1.00, 0.00, 0.30, 1.0),
    active            = Color(0.70, 0.00, 0.21, 1.0),
    background        = Color(0.15, 0.15, 0.15, 1.0),
    backgroundInvert  = Color(0.85, 0.85, 0.85, 1.0),
    border            = Color(0.00, 0.40, 1.00, 0.3),
    borderBright      = Color(1.00, 1.00, 1.00, 0.6),
    borderOverlay     = Color(0.20, 0.60, 1.00, 1.0),
    borderDim         = Color(0.50, 0.50, 0.50, 0.4),
    fill              = Color(0.60, 0.60, 0.60, 1.0),
    textNormal        = Color(0.75, 0.75, 0.75, 1.0),
    textNormalFocused = Color(0.00, 0.00, 0.00, 1.0),
    textInvert        = Color(0.25, 0.25, 0.25, 1.0),
    textInvertFocused = Color(0.00, 0.00, 0.00, 1.0),
    textTitle         = Color(0.80, 0.80, 0.80, 0.8),
    debugRect         = Color(0.50, 1.00, 0.50, 0.1),
    selection         = Color(1.00, 0.50, 0.10, 1.0),
    control           = Color(0.20, 0.90, 1.00, 1.0),
    controlFocused    = Color(0.20, 1.00, 0.20, 0.6),
    controlActive     = Color(0.14, 0.70, 0.14, 0.7),
    hologram          = Color(0.30, 0.40, 1.00, 0.8),
    ctrlCursor        = Color(0.20, 0.50, 1.00, 0.7),
    reticle           = Color(0.10, 0.30, 1.00, 3.0),
    windowBackground  = Color(0.00, 0.40, 1.00, 0.2),
    clientBackground  = Color(0.30, 0.30, 0.30, 0.0),
    meterBarOver      = Color(1.00, 0.30, 0.00, 0.6),
    meterBarLight     = Color(0.10, 0.60, 1.00, 0.7),
    meterBarDark      = Color(0.00, 0.30, 0.70, 0.1),
    meterBarBright    = Color(0.00, 0.20, 0.70, 0.3),
    hullIntegrity     = Color(0.20, 0.25, 0.30, 0.9),
    armorIntegrity    = Color(0.80, 0.75, 0.30, 0.6),
    shieldStrength    = Color(0.20, 0.50, 1.00, 0.7),
    capacitorEnergy   = Color(0.50, 0.00, 1.00, 0.7),


    healthColor = {
        --     R    G    B    A
        Color(0.0, 0.0, 0.0, 0.9), --  0% -   1% BLACK
        Color(0.1, 0.0, 0.0, 0.7), --  2% -   3%
        Color(0.2, 0.0, 0.1, 0.5), --  4% -   5%
        Color(0.3, 0.0, 0.3, 0.4), --  6% -   7%
        Color(0.4, 0.0, 0.5, 0.3), --  8% -   9%
        Color(0.5, 0.0, 0.7, 0.3), -- 10% -  11%
        Color(0.6, 0.0, 0.8, 0.2), -- 12% -  13% PURPLE
        Color(0.7, 0.0, 0.8, 0.2), -- 14% -  15%
        Color(0.8, 0.0, 0.7, 0.2), -- 16% -  17%
        Color(0.9, 0.0, 0.4, 0.2), -- 18% -  19%
        Color(1.0, 0.1, 0.2, 0.2), -- 20% -  21%
        Color(1.0, 0.1, 0.2, 0.2), -- 22% -  23%
        Color(1.0, 0.1, 0.1, 0.2), -- 24% -  25% RED
        Color(1.0, 0.2, 0.0, 0.2), -- 26% -  27%
        Color(1.0, 0.2, 0.0, 0.3), -- 28% -  29%
        Color(1.0, 0.3, 0.0, 0.3), -- 30% -  31%
        Color(0.9, 0.3, 0.0, 0.3), -- 32% -  33%
        Color(0.8, 0.4, 0.1, 0.3), -- 34% -  35%
        Color(0.8, 0.4, 0.1, 0.4), -- 36% -  37%
        Color(0.8, 0.5, 0.1, 0.5), -- 38% -  39% ORANGE
        Color(0.7, 0.5, 0.1, 0.5), -- 40% -  41%
        Color(0.7, 0.5, 0.2, 0.4), -- 42% -  43%
        Color(0.6, 0.5, 0.2, 0.3), -- 44% -  45%
        Color(0.6, 0.5, 0.2, 0.3), -- 46% -  47%
        Color(0.7, 0.6, 0.3, 0.2), -- 48% -  49%
        Color(0.7, 0.6, 0.3, 0.2), -- 50% -  51%
        Color(0.7, 0.6, 0.3, 0.2), -- 52% -  53%
        Color(0.8, 0.7, 0.4, 0.2), -- 54% -  55%
        Color(0.8, 0.7, 0.3, 0.2), -- 56% -  57%
        Color(0.8, 0.7, 0.3, 0.2), -- 58% -  59% YELLOW
        Color(0.7, 0.7, 0.2, 0.2), -- 60% -  61%
        Color(0.7, 0.7, 0.2, 0.2), -- 62% -  63%
        Color(0.6, 0.7, 0.1, 0.2), -- 64% -  65%
        Color(0.6, 0.7, 0.1, 0.2), -- 66% -  67%
        Color(0.5, 0.7, 0.0, 0.2), -- 68% -  69%
        Color(0.5, 0.8, 0.0, 0.2), -- 70% -  71%
        Color(0.4, 0.8, 0.0, 0.2), -- 72% -  73%
        Color(0.4, 0.8, 0.1, 0.2), -- 74% -  75%
        Color(0.3, 0.8, 0.1, 0.2), -- 76% -  77%
        Color(0.3, 0.8, 0.2, 0.2), -- 78% -  79% OLIVE?
        Color(0.2, 0.8, 0.2, 0.2), -- 80% -  81%
        Color(0.2, 0.9, 0.2, 0.2), -- 82% -  83%
        Color(0.2, 0.9, 0.3, 0.2), -- 84% -  85%
        Color(0.1, 0.9, 0.3, 0.2), -- 86% -  87%
        Color(0.1, 0.9, 0.2, 0.2), -- 88% -  89%
        Color(0.1, 1.0, 0.2, 0.2), -- 90% -  91%
        Color(0.1, 1.0, 0.1, 0.2), -- 92% -  93%
        Color(0.0, 1.0, 0.1, 0.2), -- 94% -  95%
        Color(0.0, 1.0, 0.0, 0.2), -- 96% -  97%
        Color(0.0, 1.0, 0.0, 0.2), -- 98% - 100% GREEN
    },
}
