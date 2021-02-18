use rand::{Rng, thread_rng};
use std::io;

static G_STRINGS: [(char, usize); 5] = [('E', 4), ('A', 0), ('D', 3), ('G', 6), ('B', 1)];
static G_NOTES: [(char, usize); 7] = [('A', 2), ('B', 1), ('C', 2), ('D', 2), ('E', 1), ('F', 2), ('G', 2)];

fn main() {
    println!("enter B for bass guitar, or just press Enter for 6-string");

    let mut bass = String::new();

    io::stdin()
        .read_line(&mut bass)
        .expect("Failed to read line");

    let mut high_range = 5;
    if &bass[..1] == "B" {
        high_range = 4;
        println!("bass\n");
    }
    else {
        println!("6-string\n");
    }

    let mut prev_question_string_index: usize = 5;
    let mut prev_question_note_index: usize = 7;
    let mut question_string_index: usize = 5;
    let mut question_note_index: usize = 7;

    loop {
        loop {
            question_string_index = rand::thread_rng().gen_range(0..high_range);
            question_note_index = rand::thread_rng().gen_range(0..7);
            if question_string_index != prev_question_string_index || question_note_index != prev_question_note_index {
                break
            };
        }

        prev_question_string_index = question_string_index;
        prev_question_note_index = question_note_index;

        println!("{}-string; note {}", G_STRINGS[question_string_index].0, G_NOTES[question_note_index].0);
        println!("enter a fret number (0-11), or Q to quit");

        let correct_fret_number = correct_fret_number(question_string_index, question_note_index);

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("failed to read line");

        if &answer[..1] == "Q" {
            println!("goodbye!\n\n");
            break;
        }

        let answer: usize = answer.trim().parse().expect("please type a number, or Q");

        if answer == correct_fret_number {
            println!("\n");
        }
        else {
            println!("**********");
            println!("no, it's at fret {}", correct_fret_number);
            println!("**********\n");
        }
    }
}

fn correct_fret_number(string_index: usize, note_index: usize) -> usize {
    let mut fret_number: usize = 0;
    let mut candidate_note_index = G_STRINGS[string_index].1;
    let mut candidate_note = G_NOTES[candidate_note_index].0;

    loop {
        // println!("fret_number: {}, candidate_note_index: {}, candidate_note: {}, adding {} frets", fret_number, candidate_note_index, candidate_note, G_NOTES[candidate_note_index].1);

        if candidate_note == G_NOTES[note_index].0 {break;}

        fret_number += G_NOTES[candidate_note_index].1;
        candidate_note_index = (candidate_note_index + 1) % 7;
        candidate_note = G_NOTES[candidate_note_index].0;
    }

    fret_number
}
