use rand::{Rng, thread_rng};

static g_strings: [(char, usize); 5] = [('E', 4), ('A', 0), ('D', 3), ('G', 6), ('B', 1)];
static g_notes: [(char, usize); 7] = [('A', 2), ('B', 1), ('C', 2), ('D', 2), ('E', 1), ('F', 2), ('G', 2)];

fn main() {

    let mut prev_question_string_index: usize = 5;
    let mut prev_question_note_index: usize = 7;
    let mut question_string_index: usize = 5;
    let mut question_note_index: usize = 7;

    for ix in 0..5
    {
        loop {
            question_string_index = rand::thread_rng().gen_range(0..5);
            question_note_index = rand::thread_rng().gen_range(0..7);
            if question_string_index != prev_question_string_index || question_note_index != prev_question_note_index {
                break
            };
        }

        prev_question_string_index = question_string_index;
        prev_question_note_index = question_note_index;

        print!("The {} string; note {}", g_strings[question_string_index].0, g_notes[question_note_index].0);

        let correct_fret_number = correct_fret_number(question_string_index, question_note_index);

        println!(" ... fret {}", correct_fret_number);
    }
}

fn correct_fret_number(string_index: usize, note_index: usize) -> usize {
    let mut fret_number: usize = 0;
    let mut candidate_note_index = g_strings[string_index].1;
    let mut candidate_note = g_notes[candidate_note_index].0;

    loop {
        // println!("fret_number: {}, candidate_note_index: {}, candidate_note: {}, adding {} frets", fret_number, candidate_note_index, candidate_note, g_notes[candidate_note_index].1);

        if candidate_note == g_notes[note_index].0 {break;}

        fret_number += g_notes[candidate_note_index].1;
        candidate_note_index = (candidate_note_index + 1) % 7;
        candidate_note = g_notes[candidate_note_index].0;
    }

    fret_number
}
