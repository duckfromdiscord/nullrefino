

pub mod board {

    #[derive(Copy, Clone)]
    pub enum TColor {
        None,
        Red,
        Orange,
        Yellow,
        Green,
        Cyan,
        Blue,
        Violet
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
                TColor::Violet => "violet"
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
                TColor::Violet => 7
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

    
}