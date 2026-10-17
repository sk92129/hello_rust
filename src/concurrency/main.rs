use std::sync::mpsc;
use std::thread;

fn transfer_numbers(numbers: Vec<i32>) -> (Vec<i32>, thread::ThreadId, thread::ThreadId) {
    let (sender, receiver) = mpsc::channel();

    let sending_thread = thread::spawn(move || {
        let thread_id = thread::current().id();
        println!("Sending thread {thread_id:?} sends {numbers:?}");
        sender
            .send(numbers)
            .expect("receiver dropped before vector was sent");
        thread_id
    });

    let receiving_thread = thread::spawn(move || {
        let thread_id = thread::current().id();
        let received_numbers = receiver
            .recv()
            .expect("sender dropped before sending vector");
        println!("Receiving thread {thread_id:?} received {received_numbers:?}");
        (received_numbers, thread_id)
    });

    let sending_thread_id = sending_thread.join().expect("sending thread panicked");
    let (received_numbers, receiving_thread_id) =
        receiving_thread.join().expect("receiving thread panicked");

    (received_numbers, sending_thread_id, receiving_thread_id)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];
    let (received_numbers, _, _) = transfer_numbers(numbers);
    println!("Vector received by the other thread: {received_numbers:?}");
}

#[cfg(test)]
mod tests {
    use super::transfer_numbers;

    #[test]
    fn sends_vector_from_one_thread_to_another() {
        let numbers = vec![1, 2, 3, 4, 5, 6];

        let (received_numbers, sending_thread_id, receiving_thread_id) = transfer_numbers(numbers);

        assert_eq!(received_numbers, vec![1, 2, 3, 4, 5, 6]);
        assert_ne!(sending_thread_id, receiving_thread_id);
    }
}
