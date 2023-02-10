



    #[derive(Copy, Clone)]
    pub enum TColor {
        None,
        Red,
        Orange,
        Yellow,
        Green,
        Cyan,
        Blue,
        Violet,
        RedClear,
        OrangeClear,
        YellowClear,
        GreenClear,
        CyanClear,
        BlueClear,
        VioletClear,
    }
    

    impl TColor {
       pub fn as_str(&self) -> &'static str {
            match self {
                TColor::None => "None",
                TColor::Red => "red",
                TColor::Orange => "orange",
                TColor::Yellow => "yellow",
                TColor::Green => "green",
                TColor::Cyan => "cyan",
                TColor::Blue => "blue",
                TColor::Violet => "violet",
                TColor::RedClear => "red",
                TColor::OrangeClear => "orange",
                TColor::YellowClear => "yellow",
                TColor::GreenClear => "green",
                TColor::CyanClear => "cyan",
                TColor::BlueClear => "blue",
                TColor::VioletClear => "violet",
            }
        }

        #[allow(dead_code)]
        pub fn to_num(clr: TColor) -> i8 {
            match clr {
                TColor::None => 0,
                TColor::Red => 1,
                TColor::Orange => 2,
                TColor::Yellow => 3,
                TColor::Green => 4,
                TColor::Cyan => 5,
                TColor::Blue => 6,
                TColor::Violet => 7,
                _ => 0,
            }
        }

        
        #[allow(non_snake_case)]
        pub fn from_etoledom(clr: &str) -> TColor {
            match clr {
               "I" => TColor::Cyan,
               "L" => TColor::Orange,
               "O" => TColor::Yellow,
               "T" => TColor::Violet,
               "S" => TColor::Green,
               "Z" => TColor::Red,
               "J" => TColor::Blue,
                _ => TColor::None,
            }
        }
        

        pub fn to_clear(&self) -> TColor {
            match self {
                TColor::None => TColor::None,
                TColor::Red => TColor::RedClear,
                TColor::Orange => TColor::OrangeClear,
                TColor::Yellow => TColor::YellowClear,
                TColor::Green => TColor::GreenClear,
                TColor::Cyan => TColor::CyanClear,
                TColor::Blue => TColor::BlueClear,
                TColor::Violet => TColor::VioletClear,
                TColor::RedClear => TColor::RedClear,
                TColor::OrangeClear => TColor::OrangeClear,
                TColor::YellowClear => TColor::YellowClear,
                TColor::GreenClear => TColor::GreenClear,
                TColor::CyanClear => TColor::CyanClear,
                TColor::BlueClear => TColor::BlueClear,
                TColor::VioletClear => TColor::VioletClear,
            }
        }
        
        pub fn is_clear(&self) -> bool {
            match self {
                TColor::None => false,
                TColor::Red => false,
                TColor::Orange => false,
                TColor::Yellow => false,
                TColor::Green => false,
                TColor::Cyan => false,
                TColor::Blue => false,
                TColor::Violet => false,
                TColor::RedClear => true,
                TColor::OrangeClear => true,
                TColor::YellowClear => true,
                TColor::GreenClear => true,
                TColor::CyanClear => true,
                TColor::BlueClear => true,
                TColor::VioletClear => true,
            }
        }

        /*
        pub fn from_etoledom(clr: Color) -> TColor {
            match clr {
               I_COLOR => TColor::Cyan,
               L_COLOR => TColor::Orange,
               O_COLOR => TColor::Yellow,
               T_COLOR => TColor::Violet,
               S_COLOR => TColor::Green,
               Z_COLOR => TColor::Red,
                _ => TColor::None,
            }
        }
        */
    }

    
