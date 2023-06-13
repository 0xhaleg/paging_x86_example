use std::fs;
use std::io::Write;

const PHYSICAL_ADDRESS_MASK: u64 = 0xFFFFFFFFFFu64 << 12;

struct TableEntry {
    paddr: u64,
    value: u64,
}

enum LogicLevels {
    Pml4 = 0x1FF<<39,
    DirectoryPtr = 0x1FF<<30,
    Directory = 0x1FF<<21,
    Table = 0x1FF<<12,
    Offset = 0xFFF,
}

fn main() {
    let input = fs::read_to_string("dataset_44327_15.txt").expect("File doesn't exist");
    let mut input = input.lines();

    let (m, q, r) = get_dataset_info(input.next().unwrap());

    let table_entries = get_table_entries(m, &mut input);

    let mut output = fs::File::create("result.txt").expect("Can not create file");

    for _ in 0..q {
        let req: u64 = input.next().unwrap().parse().unwrap();
        let ans = request(req, r, &table_entries);
        match ans {
            0 => output.write(b"fault\n").unwrap(),
            _ => output.write(&(ans.to_string()+"\n").as_bytes()).unwrap(),
        };
    }
}

fn get_dataset_info(info: &str) -> (usize, usize, u64) {
    let mut info = info.split(' ');

    (
        info.next().unwrap().parse().unwrap(),
        info.next().unwrap().parse().unwrap(),
        info.next().unwrap().parse().unwrap(),
    )
}

fn get_table_entries(entries_count: usize, entries: &mut std::str::Lines) -> Vec<TableEntry> {
    let mut result = Vec::new();

    for _ in 0..entries_count {
        let mut entries = entries
            .next().unwrap()
            .split(' ');

        let paddr: u64 = entries.next().unwrap().parse().unwrap();
        let value: u64 = entries.next().unwrap().parse().unwrap();
        result.push(TableEntry {paddr, value});
    }

    result
}

//fn get_requests(requests_count: usize, requests: &mut std::str::Lines) -> Vec<u64> {
//    let mut result = Vec::new();
//    for _ in 0..requests_count {
//        let request = requests
//            .next().unwrap()
//            .parse().unwrap();
//
//        result.push(request);
//    }
//
//    result
//}

fn request(request: u64, root_table_addr: u64, table_entries: &Vec<TableEntry>) -> u64 {
    let mut addr: u64;
    let mut value: u64;

    addr = root_table_addr + ((request & LogicLevels::Pml4 as u64) >> 39)*8;
    value = 0;
    for table_entry in table_entries {
        if table_entry.paddr == addr {
            value = table_entry.value;
            break;
        }
    }
    if (value & 0x01) == 0 {
        return 0;
    }

    addr = (value & PHYSICAL_ADDRESS_MASK) + ((request & LogicLevels::DirectoryPtr as u64) >> 30)*8;
    value = 0;
    for table_entry in table_entries {
        if table_entry.paddr == addr {
            value = table_entry.value;
            break;
        }
    }
    if (value & 0x01) == 0 {
        return 0;
    }

    addr = (value & PHYSICAL_ADDRESS_MASK) + ((request & LogicLevels::Directory as u64) >> 21)*8;
    value = 0;
    for table_entry in table_entries {
        if table_entry.paddr == addr {
            value = table_entry.value;
            break;
        }
    }
    if (value & 0x01) == 0 {
        return 0;
    }

    addr = (value & PHYSICAL_ADDRESS_MASK) + ((request & LogicLevels::Table as u64) >> 12)*8;
    value = 0;
    for table_entry in table_entries {
        if table_entry.paddr == addr {
            value = table_entry.value;
            break;
        }
    }
    if (value & 0x01) == 0 {
        return 0;
    }

    addr = (value & PHYSICAL_ADDRESS_MASK) + (request & LogicLevels::Offset as u64);

    addr
}
