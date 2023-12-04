use rltk::RGB;

// SCREEN
pub const SCREEN_WIDTH: i32 = 80;
pub const SCREEN_HEIGHT: i32 = 50;
// MAP
pub const MAP_WIDTH: i32 = 50;
pub const MAP_HEIGHT: i32 = 40;
pub const MAP_TILES: i32 = MAP_WIDTH * MAP_HEIGHT;
// MENU
pub const TITLE_Y: i32 = SCREEN_HEIGHT / 3;
pub const MENU_DELTA_Y: i32 = 2;
pub const MENU_ITEM_1_Y: i32 = SCREEN_HEIGHT / 2;
pub const TITLE_STR: &str = "OYUNA HOS GELDIN";
pub const NEW_GAME_STR: &str = "YENI OYUN";
pub const LOAD_GAME_STR: &str = "OYUN YUKLE";
pub const QUIT_GAME_STR: &str = "OYUNDAN CIK";
pub const CREDITS_STR: &str = "KATKIDA BULUNANLAR";
// CREDITS
pub const CREDIT_1_Y: i32 = SCREEN_HEIGHT / 3;
pub const CREDIT_2_Y: i32 = CREDIT_1_Y + 4;
pub const CREDIT_3_Y: i32 = CREDIT_2_Y + 4;
pub const CREDITS_THANKS_Y: i32 = CREDIT_3_Y + 10;
pub const CREDIT_1_STR: &str = "Aysila Cengiz: Fikri ve destegi icin";
pub const CREDIT_2_STR: &str = "Herbert Wolverson: Rust ile oyun gelistirmeyi ogrettigi icin";
pub const CREDIT_3_STR: &str = "Oramakoma Buramako: Tarih uzerine arastirmalari icin";
pub const CREDITS_THANKS_STR: &str = "TESEKKURLER";
// CHARS
pub const PLAYER_CHAR: char = '☻';
pub const KEY_CHAR: char = '◙';
pub const PORTAL_CHAR: char = 'Ω';
pub const NPC_CHAR: char = '☺';
// OBJECTIVE
pub const OBJECTIVE_BOX_X: i32 = INVENTORY_X;
pub const OBJECTIVE_BOX_Y: i32 = PLACE_DATE_BOX_Y - OBJECTIVE_BOX_HEIGHT - 1;
pub const OBJECTIVE_BOX_WIDTH: i32 = INVENTORY_WIDTH;
pub const OBJECTIVE_BOX_GAP: i32 = OBJECTIVE_BOX_WIDTH - 4;
pub const OBJECTIVE_BOX_HEIGHT: i32 = 10;
pub const OBJECTIVE_BANNER: &str = "Gorev";
pub const OBJECTIVE_BANNER_X: i32 = OBJECTIVE_BOX_X + OBJECTIVE_BOX_WIDTH / 2 - (OBJECTIVE_BANNER.len() / 2) as i32;
pub const OBJECTIVE_X: i32 = OBJECTIVE_BOX_X + 2;
pub const OBJECTIVE_Y: i32 = OBJECTIVE_BOX_Y + 2;
pub const OBJECTIVE_DELTA_Y: i32 = 2;
// PLACE DATE
pub const PLACE_HOME_NAME: &str = "Ev";
pub const PLACE_SCHOOL_NAME: &str = "Bogazici Guney Kampus";
pub const PLACE_CLASS_NAME: &str = "M 2152";
pub const PLACE_LIB_NAME: &str = "Bogazici Olmayan Kutuphane";
pub const PLACE_OTTOMAN_MAIN_NAME: &str = "Osmanli Meydan";
pub const PLACE_OTTOMAN_LEFT_NAME: &str = "Osmanli Sol";
pub const PLACE_OTTOMAN_RIGHT_NAME: &str = "Osmanli Sag";
pub const PLACE_OTTOMAN_TOP_NAME: &str = "Osmanli Yukari";
pub const PLACE_OTTOMAN_BOTTOM_NAME: &str = "Osmanli Asagi";
pub const CURRENT_DATE: &str = "2023";
pub const PAST_DATE: &str = "1900";
pub const PLACE_DATE_BOX_X: i32 = INVENTORY_X;
pub const PLACE_DATE_BOX_Y: i32 = INVENTORY_Y - PLACE_DATE_BOX_HEIGHT - 1;
pub const PLACE_DATE_BOX_WIDTH: i32 = INVENTORY_WIDTH;
pub const PLACE_DATE_BOX_HEIGHT: i32 = 6;
pub const PLACE_DATE_BOX_GAP: i32 = PLACE_DATE_BOX_WIDTH - 4;
pub const PLACE_DATE_BANNER: &str = "Mekan/Zaman";
pub const PLACE_DATE_BANNER_X: i32 = PLACE_DATE_X + PLACE_DATE_BOX_WIDTH / 2 - (PLACE_DATE_BANNER.len() / 2) as i32;
pub const PLACE_DATE_X: i32 = PLACE_DATE_BOX_X + 2;
pub const PLACE_DATE_Y: i32 = PLACE_DATE_BOX_Y + 2;
pub const PLACE_DATE_DELTA_Y: i32 = 2;
// INVENTORY
pub const INVENTORY_X: i32 = MAP_WIDTH;
pub const INVENTORY_Y: i32 = MAP_HEIGHT - INVENTORY_HEIGHT - 1;
pub const INVENTORY_DELTA_Y: i32 = 2;
pub const INVENTORY_WIDTH: i32 = SCREEN_WIDTH - MAP_WIDTH - 1;
pub const INVENTORY_HEIGHT: i32 = 19;
pub const INVENTORY_BANNER: &str = "Esyalar";
pub const INVENTORY_BANNER_X: i32 = INVENTORY_X + INVENTORY_WIDTH / 2 - (INVENTORY_BANNER.len() / 2) as i32;
pub const INVENTORY_ITEMS_X: i32 = INVENTORY_X + 2 * INVENTORY_DELTA_Y;
pub const INVENTORY_ITEMS_Y: i32 = INVENTORY_Y + INVENTORY_DELTA_Y;
// ITEMS
pub const ITEM_BOOK_NAME: &str = "Taylan Hoca'nin Kitabi";
pub const ITEM_SECRET_GATE_KEY_NAME: &str = "Gizli Gecit Anahtari";
pub const OTTOMAN_KEY_1_NAME: &str = "Kapi 1 Anahtari";
pub const OTTOMAN_REWARD_1_NAME: &str = "Odul 1";
pub const OTTOMAN_KEY_2_NAME: &str = "Kapi 2 Anahtari";
pub const OTTOMAN_REWARD_2_NAME: &str = "Odul 2";
pub const OTTOMAN_KEY_3_NAME: &str = "Kapi 3 Anahtari";
pub const OTTOMAN_REWARD_3_NAME: &str = "Odul 3";
pub const OTTOMAN_KEY_4_NAME: &str = "Kapi 4 Anahtari";
pub const OTTOMAN_REWARD_4_NAME: &str = "Odul 4";
pub const OTTOMAN_KEY_MAIN_NAME: &str = "Zaman Kapisi Anahtari";
// NPC INTERACTION
pub const NPC_INTERACTION_SCREEN_WIDTH: i32 = 40;
pub const NPC_INTERACTION_SCREEN_HEIGHT: i32 = 40;
pub const NPC_INTERACTION_SCREEN_GAP_WIDTH: i32 = NPC_INTERACTION_SCREEN_WIDTH - 3 * NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_SCREEN_X: i32 = (MAP_WIDTH - NPC_INTERACTION_SCREEN_WIDTH) / 2;
pub const NPC_INTERACTION_SCREEN_Y: i32 = (MAP_HEIGHT - NPC_INTERACTION_SCREEN_HEIGHT) / 2;
pub const NPC_INTERACTION_DIALOGUE_DELTA: i32 = 2;
pub const NPC_INTERACTION_DIALOGUE_HEADING_X: i32 = NPC_INTERACTION_SCREEN_X + NPC_INTERACTION_SCREEN_WIDTH / 2;
pub const NPC_INTERACTION_DIALOGUE_HEADING_Y: i32 = NPC_INTERACTION_SCREEN_Y + NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_GLYPH_X: i32 = NPC_INTERACTION_SCREEN_X + NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_DIALOGUE_X: i32 = NPC_INTERACTION_GLYPH_X + NPC_INTERACTION_DIALOGUE_DELTA;
pub const NPC_INTERACTION_DIALOGUE_Y: i32 = NPC_INTERACTION_DIALOGUE_HEADING_Y + NPC_INTERACTION_DIALOGUE_DELTA;
// ROOMS
// HOME
pub const HOME_X: i32 = (MAP_WIDTH - HOME_WIDTH) / 2;
pub const HOME_Y: i32 = (MAP_HEIGHT - HOME_HEIGHT) / 2;
pub const HOME_WIDTH: i32 = 10;
pub const HOME_HEIGHT: i32 = 10;
pub const HOME_TO_SCHOOL_PORTAL_COORD: (i32, i32) = (HOME_X + HOME_WIDTH / 2, HOME_Y - 1);
pub const HOME_FROM_SCHOOL_COORD: (i32, i32) = (HOME_TO_SCHOOL_PORTAL_COORD.0, HOME_TO_SCHOOL_PORTAL_COORD.1 + 1);
// SCHOOL
pub const SCHOOL_X: i32 = (MAP_WIDTH - SCHOOL_WIDTH) / 2;
pub const SCHOOL_Y: i32 = (MAP_HEIGHT - SCHOOL_HEIGHT) / 2;
pub const SCHOOL_WIDTH: i32 = 25;
pub const SCHOOL_HEIGHT: i32 = 20;
pub const SCHOOL_TO_CLASS_PORTAL_COORD: (i32, i32) = (SCHOOL_X + SCHOOL_WIDTH, SCHOOL_Y + SCHOOL_HEIGHT / 2);
pub const SCHOOL_FROM_CLASS_COORD: (i32, i32) = (SCHOOL_TO_CLASS_PORTAL_COORD.0 - 1, SCHOOL_TO_CLASS_PORTAL_COORD.1);
pub const SCHOOL_TO_HOME_PORTAL_COORD: (i32, i32) = (SCHOOL_X + SCHOOL_WIDTH / 2, SCHOOL_Y + SCHOOL_HEIGHT);
pub const SCHOOL_FROM_HOME_COORD: (i32, i32) = (SCHOOL_TO_HOME_PORTAL_COORD.0, SCHOOL_TO_HOME_PORTAL_COORD.1 - 1);
pub const SCHOOL_TO_LIBRARY_PORTAL_COORD: (i32, i32) = (SCHOOL_X - 1, SCHOOL_Y + SCHOOL_HEIGHT / 2);
pub const SCHOOL_FROM_LIBRARY_COORD: (i32, i32) = (SCHOOL_TO_LIBRARY_PORTAL_COORD.0 + 1, SCHOOL_TO_LIBRARY_PORTAL_COORD.1);
pub const SCHOOL_TO_OTTOMAN_PORTAL_COORD: (i32, i32) = (SCHOOL_X + SCHOOL_WIDTH / 2, SCHOOL_Y - 1);
pub const SCHOOL_FROM_OTTOMAN_COORD: (i32, i32) = (SCHOOL_TO_OTTOMAN_PORTAL_COORD.0, SCHOOL_TO_OTTOMAN_PORTAL_COORD.1 + 1);
// CLASS
pub const CLASS_X: i32 = (MAP_WIDTH - CLASS_WIDTH) / 2;
pub const CLASS_Y: i32 = (MAP_HEIGHT - CLASS_HEIGHT) / 2;
pub const CLASS_WIDTH: i32 = 12;
pub const CLASS_HEIGHT: i32 = 18;
pub const CLASS_TO_SCHOOL_PORTAL_COORD: (i32, i32) = (CLASS_X - 1, CLASS_Y + 3);
pub const CLASS_FROM_SCHOOL_COORD: (i32, i32) = (CLASS_TO_SCHOOL_PORTAL_COORD.0 + 1, CLASS_TO_SCHOOL_PORTAL_COORD.1);
// LIBRARY
pub const LIBRARY_X: i32 = (MAP_WIDTH - LIBRARY_WIDTH) / 2;
pub const LIBRARY_Y: i32 = (MAP_HEIGHT - LIBRARY_HEIGHT) / 2;
pub const LIBRARY_WIDTH: i32 = 14;
pub const LIBRARY_HEIGHT: i32 = 14;
pub const LIBRARY_TO_SCHOOL_PORTAL_COORD: (i32, i32) = (LIBRARY_X + LIBRARY_WIDTH, LIBRARY_Y + LIBRARY_HEIGHT / 2);
pub const LIBRARY_FROM_SCHOOL_COORD: (i32, i32) = (LIBRARY_TO_SCHOOL_PORTAL_COORD.0 - 1, LIBRARY_TO_SCHOOL_PORTAL_COORD.1);
// OTTOMAN MAIN
pub const OTTOMAN_MAIN_X: i32 = (MAP_WIDTH - OTTOMAN_MAIN_WIDTH) / 2;
pub const OTTOMAN_MAIN_Y: i32 = (MAP_HEIGHT - OTTOMAN_MAIN_HEIGHT) / 2;
pub const OTTOMAN_MAIN_WIDTH: i32 = 18;
pub const OTTOMAN_MAIN_HEIGHT: i32 = 18;
pub const OTTOMAN_TO_SCHOOL_PORTAL_COORD: (i32, i32) = (OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH / 2 - 2, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT / 2);
pub const OTTOMAN_FROM_SCHOOL_COORD: (i32, i32) = (OTTOMAN_TO_SCHOOL_PORTAL_COORD.0 + 1, OTTOMAN_TO_SCHOOL_PORTAL_COORD.1);
pub const OTTOMAN_TO_LEFT_PORTAL_COORD: (i32, i32) = (OTTOMAN_MAIN_X - 1, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT / 2);
pub const OTTOMAN_FROM_LEFT_COORD: (i32, i32) = (OTTOMAN_TO_LEFT_PORTAL_COORD.0 + 1, OTTOMAN_TO_LEFT_PORTAL_COORD.1);
pub const OTTOMAN_TO_TOP_PORTAL_COORD: (i32, i32) = (OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH / 2, OTTOMAN_MAIN_Y - 1);
pub const OTTOMAN_FROM_TOP_COORD: (i32, i32) = (OTTOMAN_TO_TOP_PORTAL_COORD.0, OTTOMAN_TO_TOP_PORTAL_COORD.1 + 1);
pub const OTTOMAN_TO_RIGHT_PORTAL_COORD: (i32, i32) = (OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT / 2);
pub const OTTOMAN_FROM_RIGHT_COORD: (i32, i32) = (OTTOMAN_TO_RIGHT_PORTAL_COORD.0 - 1, OTTOMAN_TO_RIGHT_PORTAL_COORD.1);
pub const OTTOMAN_TO_BOTTOM_PORTAL_COORD: (i32, i32) = (OTTOMAN_MAIN_X + OTTOMAN_MAIN_WIDTH / 2, OTTOMAN_MAIN_Y + OTTOMAN_MAIN_HEIGHT);
pub const OTTOMAN_FROM_BOTTOM_COORD: (i32, i32) = (OTTOMAN_TO_BOTTOM_PORTAL_COORD.0, OTTOMAN_TO_BOTTOM_PORTAL_COORD.1 - 1);
// OTTOMAN LEFT
pub const OTTOMAN_LEFT_X: i32 = (MAP_WIDTH - OTTOMAN_LEFT_WIDTH) / 2;
pub const OTTOMAN_LEFT_Y: i32 = (MAP_HEIGHT - OTTOMAN_LEFT_HEIGHT) / 2;
pub const OTTOMAN_LEFT_WIDTH: i32 = 12;
pub const OTTOMAN_LEFT_HEIGHT: i32 = 12;
pub const OTTOMAN_LEFT_TO_MAIN_PORTAL_COORD: (i32, i32) = (OTTOMAN_LEFT_X + OTTOMAN_LEFT_WIDTH, OTTOMAN_LEFT_Y + OTTOMAN_LEFT_HEIGHT / 2);
pub const OTTOMAN_LEFT_FROM_MAIN_COORD: (i32, i32) = (OTTOMAN_LEFT_TO_MAIN_PORTAL_COORD.0 - 1, OTTOMAN_LEFT_TO_MAIN_PORTAL_COORD.1);
// OTTOMAN TOP
pub const OTTOMAN_TOP_X: i32 = (MAP_WIDTH - OTTOMAN_TOP_WIDTH) / 2;
pub const OTTOMAN_TOP_Y: i32 = (MAP_HEIGHT - OTTOMAN_TOP_HEIGHT) / 2;
pub const OTTOMAN_TOP_WIDTH: i32 = 12;
pub const OTTOMAN_TOP_HEIGHT: i32 = 12;
pub const OTTOMAN_TOP_TO_MAIN_PORTAL_COORD: (i32, i32) = (OTTOMAN_TOP_X + OTTOMAN_TOP_WIDTH / 2, OTTOMAN_TOP_Y + OTTOMAN_TOP_HEIGHT);
pub const OTTOMAN_TOP_FROM_MAIN_COORD: (i32, i32) = (OTTOMAN_TOP_TO_MAIN_PORTAL_COORD.0, OTTOMAN_TOP_TO_MAIN_PORTAL_COORD.1 - 1);
// OTTOMAN RIGHT
pub const OTTOMAN_RIGHT_X: i32 = (MAP_WIDTH - OTTOMAN_RIGHT_WIDTH) / 2;
pub const OTTOMAN_RIGHT_Y: i32 = (MAP_HEIGHT - OTTOMAN_RIGHT_HEIGHT) / 2;
pub const OTTOMAN_RIGHT_WIDTH: i32 = 12;
pub const OTTOMAN_RIGHT_HEIGHT: i32 = 12;
pub const OTTOMAN_RIGHT_TO_MAIN_PORTAL_COORD: (i32, i32) = (OTTOMAN_RIGHT_X - 1, OTTOMAN_RIGHT_Y + OTTOMAN_LEFT_HEIGHT / 2);
pub const OTTOMAN_RIGHT_FROM_MAIN_COORD: (i32, i32) = (OTTOMAN_RIGHT_TO_MAIN_PORTAL_COORD.0 + 1, OTTOMAN_RIGHT_TO_MAIN_PORTAL_COORD.1);
// OTTOMAN BOTTOM
pub const OTTOMAN_BOTTOM_X: i32 = (MAP_WIDTH - OTTOMAN_BOTTOM_WIDTH) / 2;
pub const OTTOMAN_BOTTOM_Y: i32 = (MAP_HEIGHT - OTTOMAN_BOTTOM_HEIGHT) / 2;
pub const OTTOMAN_BOTTOM_WIDTH: i32 = 12;
pub const OTTOMAN_BOTTOM_HEIGHT: i32 = 12;
pub const OTTOMAN_BOTTOM_TO_MAIN_PORTAL_COORD: (i32, i32) = (OTTOMAN_BOTTOM_X + OTTOMAN_BOTTOM_WIDTH / 2, OTTOMAN_BOTTOM_Y - 1);
pub const OTTOMAN_BOTTOM_FROM_MAIN_COORD: (i32, i32) = (OTTOMAN_BOTTOM_TO_MAIN_PORTAL_COORD.0, OTTOMAN_BOTTOM_TO_MAIN_PORTAL_COORD.1 + 1);
// COLORS
pub const BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
// COLORS MENU
pub const MENU_SELECTED_COLOR: RGB = RGB { r: 1.0, g: 0., b: 0. };
pub const MENU_UNSELECTED_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS CREDITS
pub const CREDITS_1_COLOR: RGB = RGB { r: 1.0, g: 0.0, b: 0.0 };
pub const CREDITS_2_COLOR: RGB = RGB { r: 0.0, g: 1.0, b: 0.0 };
pub const CREDITS_3_COLOR: RGB = RGB { r: 0.0, g: 0.0, b: 1.0 };
pub const CREDITS_THANKS_COLOR: RGB = RGB { r: 0.2, g: 0.4, b: 0.6 };
// COLORS ITEMS
pub const ITEM_KEY_COLOR: RGB = RGB { r: 240f32 / 255.0, g: 250f32 / 255.0, b: 30f32 / 255.0 };
pub const ITEM_DOOR_COLOR: RGB = RGB { r: 70f32 / 255.0, g: 200f32 / 255.0, b: 200f32 / 255.0 };
pub const ITEM_PORTAL_COLOR: RGB = RGB { r: 21f32 / 255.0, g: 246f32 / 255.0, b: 111f32 / 255.0 };
// COLORS TILES
pub const SPACE_COLOR: RGB = RGB { r: 131f32 / 255.0, g: 131f32 / 255.0, b: 131f32 / 255.0 };
pub const TILE_COLOR: RGB = RGB { r: 188f32 / 255.0, g: 188f32 / 255.0, b: 188f32 / 255.0 };
pub const WALL_COLOR: RGB = RGB { r: 130f32 / 255.0, g: 130f32 / 255.0, b: 130f32 / 255.0 };
// COLORS CURSOR
pub const CURSOR_COLOR: RGB = RGB { r: 242f32 / 255.0, g: 47f32 / 255.0, b: 196f32 / 255.0 };
// COLORS CONSOLE
pub const CONSOLE_BORDER_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const CONSOLE_BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
pub const CONSOLE_LOG_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS OBJECTIVE
pub const OBJECTIVE_BOX_FG: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const OBJECTIVE_BOX_BG: RGB = RGB { r: 0., g: 0., b: 0. };
pub const OBJECTIVE_BANNER_COLOR: RGB = RGB { r: 238f32 / 255.0, g: 253f32 / 255.0, b: 28f32 / 255.0 };
// COLORS PLACE DATE
pub const PLACE_DATE_BOX_FG: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const PLACE_DATE_BOX_BG: RGB = RGB { r: 0., g: 0., b: 0. };
pub const PLACE_DATE_BANNER_COLOR: RGB = RGB { r: 238f32 / 255.0, g: 253f32 / 255.0, b: 28f32 / 255.0 };
pub const PLACE_DATE_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS INVENTORY
pub const INVENTORY_BORDER_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const INVENTORY_BACKGROUND_COLOR: RGB = RGB { r: 0., g: 0., b: 0. };
pub const INVENTORY_BANNER_COLOR: RGB = RGB { r: 238f32 / 255.0, g: 253f32 / 255.0, b: 28f32 / 255.0 };
pub const INVENTORY_STRING_COLOR: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
// COLORS NPC INTERACTION
pub const NPC_INTERACTION_SCREEN_FG: RGB = RGB { r: 1.0, g: 1.0, b: 1.0 };
pub const NPC_INTERACTION_SCREEN_BG: RGB = RGB { r: 0.0, g: 0.0, b: 0.0 };
// COLORS CHARACTERS
pub const PLAYER_COLOR: RGB = RGB { r: 1.0, g: 50f32 / 255.0, b: 0. };
pub const NPC_COLOR: RGB = RGB { r: 1.0, g: 111f32 / 255.0, b: 0. };




