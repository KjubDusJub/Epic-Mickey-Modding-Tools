use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;
use std::env;
extern crate bytes;
use bytes::{BytesMut, BufMut};


fn is_even(value: i32) -> bool { 
    return value % 2 == 0;
}

fn write_bytes(source: Vec<u8>, new: Vec<u8>, index: i32) -> Vec<u8> 
{
    let mut new_value = source;
    let mut current_index = index;

    for byte in new {
         new_value.remove(current_index.try_into().unwrap());
         new_value.insert(current_index.try_into().unwrap(), byte);
                current_index += 1;
    }

    return new_value;
}


fn main() {

    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    let mut source = PathBuf::new();
    source.push(&args[1]);

    

    let f = File::open(source).expect("Couldn't open source file");
    let mut reader = BufReader::new(f);
    let mut full_buffer = Vec::new();
    reader.read_to_end(&mut full_buffer).expect("couldn't read bytes from file");

    let sample_rate = &full_buffer[8..12];

    let samples = &full_buffer[0x60..0x64];

    let file_size_full = full_buffer.len();

    let mut is_stereo = true;

    let b = &full_buffer[0xB0..0xB1];
     
    if b[0].to_string() != "0" {
          println!("mono!!");
          is_stereo = false;
    }
    
    let mut header_end = 0xC0;

    if !is_stereo {
        header_end = 0x60;
    }

    let buffer = full_buffer[header_end..file_size_full].to_vec();

    let file_size = buffer.len();

    // seek bytes like this &buffer[0x3E..0x53];

    let chunks = buffer.chunks(0x2000);

    let mut i = 0;

    let mut even_bytes = BytesMut::new();
    let mut odd_bytes = BytesMut::new();
    let mut final_bytes = BytesMut::new();

    for chunk in chunks {
       if is_even(i) {
        even_bytes.put(chunk);
       }
       else {
        odd_bytes.put(chunk);
       }
       i += 1;
    }

    if is_stereo
    {
        let mut even_chunks = even_bytes.chunks(0x8);
        let mut odd_chunks =  odd_bytes.chunks(0x8);
        
        let iterations = file_size / 0x8;
    
        for n in 0..iterations {
    
            if !is_even(n.try_into().unwrap()) {
                if odd_chunks.clone().peekable().peek().is_some() {
                    let bytes = odd_chunks.next().unwrap();
                    final_bytes.put(bytes);
    
                }    
            }
            else{
                if even_chunks.clone().peekable().peek().is_some() {
                    let bytes = even_chunks.next().unwrap();
                    final_bytes.put(bytes);
    
               
                }
            }
         
        }
    }
    else{

        final_bytes.put(buffer.as_slice());
    }


    let mut header_filename = "header";

    if !is_stereo
    {
        header_filename = "headermono";
    }

    let f = File::open(header_filename).expect("Couldn't open header file");

    let mut header_reader = BufReader::new(f);
    let mut header_buffer = Vec::new();
    header_reader.read_to_end(&mut header_buffer).expect("couldn't read bytes from file");

    //todo: rename these since channel two is supposed to be channel one
    let channel_one_data = &full_buffer[0x7C..0xA9];
    let channel_two_data = &full_buffer[0x1C..0x49];

  

     let mut final_header = write_bytes(header_buffer, channel_two_data.to_vec(), 0x88);

     let data_tag_bypass = 35457240_i32.to_be_bytes();

     if is_stereo {
        final_header = write_bytes(final_header, channel_one_data.to_vec(), 0xB6);
        final_header = write_bytes(final_header, samples.to_vec(), 0x74);
        final_header = write_bytes(final_header, data_tag_bypass.to_vec(), 0xFC);   
        final_header = write_bytes(final_header, samples.to_vec(), 0x1C);   
        final_header = write_bytes(final_header, sample_rate.to_vec(), 0x18);   
     }
     else{
        final_header = write_bytes(final_header, data_tag_bypass.to_vec(), 0xDC);
        final_header = write_bytes(final_header, samples.to_vec(), 0x74);   
        final_header = write_bytes(final_header, samples.to_vec(), 0x1C);   
        final_header = write_bytes(final_header, sample_rate.to_vec(), 0x18);   
     }
    
    final_header.append(&mut final_bytes.to_vec());
    let mut file = File::create("temp").unwrap();
    file.write_all(&final_header).unwrap();

}
