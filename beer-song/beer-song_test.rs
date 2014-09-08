#![crate_name = "beer-song_test"]
#![crate_type = "lib"]

mod beer;

#[test]
#[ignore]
fn test_verse_0() {
    assert_eq!(beer::verse(0).as_slice(), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}

#[test]
#[ignore]
fn test_verse_1() {
    assert_eq!(beer::verse(1).as_slice(), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
}

#[test]
#[ignore]
fn test_verse_2() {
    assert_eq!(beer::verse(2).as_slice(), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
}

#[test]
#[ignore]
fn test_verse_8() {
    assert_eq!(beer::verse(8).as_slice(), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
}

#[test]
#[ignore]
fn test_song_8_6() {
    assert_eq!(beer::sing(8, 6).as_slice(), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
}

#[test]
#[ignore]
fn test_song_3_0() {
    assert_eq!(beer::sing(3, 0).as_slice(), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
}
