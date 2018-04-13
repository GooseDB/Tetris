use coord::Coord;
use color::Color;

pub const COLOR_I: (Color, Color) = (Color::new(127, 105, 113), Color::new(143, 117, 126));
pub const COLOR_J: (Color, Color) = (Color::new(127, 105, 127), Color::new(142, 117, 142));
pub const COLOR_L: (Color, Color) = (Color::new(116, 105, 127), Color::new(130, 117, 142));
pub const COLOR_O: (Color, Color) = (Color::new(105, 105, 127), Color::new(118, 117, 142));
pub const COLOR_S: (Color, Color) = (Color::new(104, 114, 127), Color::new(117, 128, 142));
pub const COLOR_T: (Color, Color) = (Color::new(106, 125, 128), Color::new(117, 140, 141));
pub const COLOR_Z: (Color, Color) = (Color::new(105, 127, 117), Color::new(117, 142, 131));

pub const I: [[Coord; 4]; 2] = [
    /*
    + - - - -
    | # # # #
    |
    |
     */
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(3, 0),
    ],
    /*
    + - - - -
    | #
    | #
    | #
    | #
     */
    [
        Coord::new(0, 0),
        Coord::new(0, 1),
        Coord::new(0, 2),
        Coord::new(0, 3),
    ],
];

pub const J: [[Coord; 4]; 4] = [
    /*
    + - - - -
    | # 
    | # # #
    |
     */
    [
        Coord::new(0, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(2, 1),
    ],
    /*
    + - - - -
    |   # #
    |   #
    |   #
     */
    [
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(1, 1),
        Coord::new(1, 2),
    ],
    /*
    + - - - -
    | # # #
    |     #
    |
     */
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(2, 1),
    ],
    /*
    + - - - -
    |   #
    |   #
    | # #
     */
    [
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(0, 2),
        Coord::new(1, 2),
    ],
];

pub const L: [[Coord; 4]; 4] = [
    /*
    + - - - -
    | # # #
    | #
    |
     */
    [
        Coord::new(0, 1),
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
    ],
    /*
    + - - - -
    | # #
    |   # 
    |   #
     */
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(1, 2),
    ],
    /*
    + - - - -
    |     #
    | # # #
    |
     */
    [
        Coord::new(2, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(2, 1),
    ],
    /*
    + - - - -
    |   #
    |   #
    |   # #
     */
    [
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(1, 2),
        Coord::new(2, 2),
    ],
];

pub const O: [[Coord; 4]; 1] = [
    /*
    + - - - -
    | # #
    | # #
    |   
     */
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
    ],
];

pub const S: [[Coord; 4]; 2] = [
    /*
    + - - - -   
    |   # #
    | # #
    |
     */
    [
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
    ],
    /*
    + - - - -
    |   #  
    |   # #
    |     #
     */
    [
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(2, 1),
        Coord::new(2, 2),
    ],
];

pub const T: [[Coord; 4]; 4] = [
    /*
    + - - - -
    |   #
    | # # #
    |
     */
    [
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(2, 1),
    ],
    /*
    + - - - -
    | #
    | # #
    | #
     */
    [
        Coord::new(0, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(0, 2),
    ],
    /*
    + - - - -  
    | # # #
    |   #
    |
     */
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(2, 0),
        Coord::new(1, 1),
    ],
    /*
    + - - - -  
    |     #
    |   # #
    |     #
    */
    [
        Coord::new(2, 0),
        Coord::new(1, 1),
        Coord::new(2, 1),
        Coord::new(2, 2),
    ],
];

pub const Z: [[Coord; 4]; 2] = [
    /*
    + - - - -  
    | # #
    |   # # 
    |
    */
    [
        Coord::new(0, 0),
        Coord::new(1, 0),
        Coord::new(1, 1),
        Coord::new(2, 1),
    ],
    /*
    + - - - -  
    |   #
    | # # 
    | #
    */
    [
        Coord::new(1, 0),
        Coord::new(0, 1),
        Coord::new(1, 1),
        Coord::new(0, 2),
    ],
];
