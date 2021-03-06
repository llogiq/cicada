use board;
use zobrist;

pub const diagonal : [i8; 4] = [11, 9, -9, -11];
pub const knight : [i8; 8] = [8, 19, 21, 12, -8, -19, -21, -12];
pub const cross : [i8; 4] = [10, 1, -10, -1];
pub const king : [i8; 8] = [9, 10, 11, 1, -9, -10, -11, -1];

pub fn attacked(target : u8, side : bool, cboard : &board::chessboard) -> bool {
    let mut search : u8;
    let mut queen : u8;

    // attack by pawns
    if side == board::white {
        search = board::piece::P as u8;
        for x in 2..4 {
            if cboard.layout[(target as i8 + diagonal[x]) as usize] == search {
                return true;
            }
        }
    }
    else {
        search = board::piece::p as u8;
        for x in 0..2 {
            if cboard.layout[(target as i8 + diagonal[x]) as usize] == search {
                return true;
            }
        }
    }

    // attack by knights
    if side == board::white {search = board::piece::N as u8;} 
    else {search = board::piece::n as u8;}
    for x in 0..8 {
        if cboard.layout[(target as i8 + knight[x]) as usize] == search {
            return true;
        }
    }

    // attack by bishop or queen diagonally
    if side == board::white {search = board::piece::B as u8; queen = board::piece::Q as u8;} 
    else {search = board::piece::b as u8; queen = board::piece::q as u8;}
    for x in 0..4 {
        let mut current = target as usize;
        current = (current as i8 + diagonal[x]) as usize;
        while cboard.layout[current] != board::void_square {
            if cboard.layout[current] == search || cboard.layout[current] == queen {
                return true;
            } else if cboard.layout[current] != board::piece::Empty as u8 {
                // blocking piece
                break;
            }
            current = (current as i8 + diagonal[x]) as usize;
        }
    }

    // attack by rook or cross queen
    if side == board::white {search = board::piece::R as u8; queen = board::piece::Q as u8;} 
    else {search = board::piece::r as u8; queen = board::piece::q as u8;}
    for x in 0..4 {
        let mut current = target as usize;
        current = (current as i8 + cross[x]) as usize;
        while cboard.layout[current] != board::void_square {
            if cboard.layout[current] == search || cboard.layout[current] == queen {
                return true;
            } else if cboard.layout[current] != board::piece::Empty as u8 {
                // blocking piece
                break;
            }
            current = (current as i8 + cross[x]) as usize;
        }
    }
    
    // attack by king
    if side == board::white {search = board::piece::K as u8;} 
    else {search = board::piece::k as u8;}
    for x in 0..8 {
        if cboard.layout[(target as i8 + king[x]) as usize] == search {
            return true;
        }
    }

    // no attack found
    false
}

pub fn clear (target : u8, cboard : &mut board::chessboard) {
    debug_assert!(cboard.layout[target as usize] != board::piece::Empty as u8);

    // unhash target - TODO unsure this will work, need to hash in empty?
    // should be fixed now
    zobrist::hash_square(target, cboard);

    let kind = cboard.layout[target as usize] as usize;
    for x in 0..cboard.piece_count[kind] as usize {
        if cboard.piece_list[kind][x] == target {
            cboard.piece_list[kind][x] = cboard.piece_list[kind][cboard.piece_count[kind] as usize - 1];
            break;
        }
    }
    cboard.piece_count[kind] -= 1;
    if kind < 7 {
        // white
        cboard.score[0] -= board::piece_value[kind as usize];
    } else {
        cboard.score[1] -= board::piece_value[kind as usize];
    }

    cboard.layout[target as usize] = board::piece::Empty as u8;
}

pub fn add (target : u8, kind : u8, cboard : &mut board::chessboard) {
    debug_assert!(cboard.layout[target as usize] == board::piece::Empty as u8);

    cboard.layout[target as usize] = kind;

    cboard.piece_list[kind as usize][cboard.piece_count[kind as usize] as usize] = target;
    cboard.piece_count[kind as usize] += 1;
    if kind < 7 {
        // white
        cboard.score[0] += board::piece_value[kind as usize];
    } else {
        cboard.score[1] += board::piece_value[kind as usize];
    }

    // TODO could make this call cheaper
    zobrist::hash_square(target, cboard);
}

pub fn plsmove (origin : u8, target : u8, cboard : &mut board::chessboard) {
    debug_assert!(cboard.layout[target as usize] == board::piece::Empty as u8);

    // hash out square
    zobrist::hash_square(origin, cboard);

    let kind = cboard.layout[origin as usize] as usize;
    for x in 0..cboard.piece_count[kind] as usize {
        if cboard.piece_list[kind][x] == origin {
            cboard.piece_list[kind][x] = target;
            break;
        }
    }

    cboard.layout[target as usize] = kind as u8;
    cboard.layout[origin as usize] = board::piece::Empty as u8;

    // hash in new square
    zobrist::hash_square(target, cboard);
}
