extern crate tools;
use tools::read_from_file;

extern crate regex;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Event {
    timestamp: Timestamp,
    eventtype: EventType,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Timestamp {
    y: u32,
    m: u32,
    d: u32,
    hour: u32,
    min: u32,
}

#[derive(Debug)]
enum EventType {
    BeginShift { id: u32 },
    FallAsleep,
    WakeUp,
}

fn line_to_event(line: &str) -> Option<Event> {
    let re = Regex::new(
        r"(?x)^\[(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})\s
          (?P<hour>\d{2}):(?P<min>\d{2})\]\s
          (?:Guard\s\#(?P<id>\d+)\sbegins.* | (?P<change>.*))$",
    ).unwrap();

    let cap = re.captures(line).unwrap();

    let timestamp = Timestamp {
        y: cap["y"].parse().unwrap(),
        m: cap["m"].parse().unwrap(),
        d: cap["d"].parse().unwrap(),
        hour: cap["hour"].parse().unwrap(),
        min: cap["min"].parse().unwrap(),
    };

    let eventtype = if let Some(idstr) = cap.name("id") {
        EventType::BeginShift {
            id: idstr.as_str().parse().unwrap(),
        }
    } else if &cap["change"] == "falls asleep" {
        EventType::FallAsleep
    } else if &cap["change"] == "wakes up" {
        EventType::WakeUp
    } else {
        return None;
    };

    Some(Event {
        timestamp,
        eventtype,
    })
}

fn main() {
    let input = read_from_file("input").unwrap();

    let mut events: Vec<Event> = Vec::new();

    for l in input.lines() {
        events.push(line_to_event(l).unwrap());
    }

    // mapping guard id to minute to frequency
    let mut guardminfreq: HashMap<u32, HashMap<u32, u32>> = HashMap::new();
    events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    let mut thisguard = 0;
    let mut sleepingsince: Option<Timestamp> = None;
    for e in events {
        match e.eventtype {
            EventType::BeginShift { id } => {
                thisguard = id;
                sleepingsince = None;
            }
            EventType::FallAsleep => {
                sleepingsince = Some(e.timestamp);
            }
            EventType::WakeUp => {
                let minfreq = &mut (*guardminfreq.entry(thisguard).or_default());
                for min in sleepingsince.unwrap().min..e.timestamp.min {
                    *minfreq.entry(min).or_default() += 1;
                }
            }
        }
    }

    // pick out id of guard sleeping the most
    let (id, minfreq) = guardminfreq
        .iter()
        .max_by_key(|&(_, minfreq)| minfreq.values().sum::<u32>())
        .unwrap();

    // find the minute that this guard most frequently sleeps on
    let (min, _) = minfreq.iter().max_by_key(|&(_, freq)| freq).unwrap();

    println!("part1, {:?} * {:?} = {:?}", id, min, id * min);

    // find the minute that is most frequently slept on by any guard
    let mut theguard = 0;
    let mut themin = 0;
    let mut freqhigh = 0;
    for (id, minfreq) in guardminfreq.iter() {
        let (min, freq) = minfreq.iter().max_by_key(|&(_, freq)| freq).unwrap();
        if *freq > freqhigh {
            theguard = *id;
            themin = *min;
            freqhigh = *freq
        }
    }

    println!(
        "part2, {:?} * {:?} = {:?}",
        theguard,
        themin,
        theguard * themin
    );
}
