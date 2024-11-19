use rand::Rng;

struct Bus {
    bus_number: u8,
    route_number: u8,
    driver: Driver,
    num_stops: u8,
    current_stop: u8,
    passengers: Vec<Passenger>,
}

impl Bus {
    fn next_stop(&mut self) {
        if self.current_stop < self.num_stops {
            self.current_stop += 1;
        } else {
            self.current_stop = 1;
        }
    }

    fn passengers_off(&mut self) {
        self.passengers.retain(|passenger| passenger.stop_off != self.current_stop)
    }

    fn passengers_on(&mut self, rng) {
        let num_new_passengers: u8 = rng.gen_range(0..=5);

        for _ in 0..=num_new_passengers {
            let new_passenger = Passenger {
                name: String::from("Thomas"),
                stop_off: rng.gen_range(1..=self.num_stops),
            };
            self.passengers.push(new_passenger)
        }
    }
}

struct Driver {
    name: String,
}

struct Passenger {
    name: String,
    stop_off: u8,
}

impl Passenger {

}

fn main() {
    let mut rng = rand::thread_rng();

    let driver = Driver {
        name: String::from("Dan"),
    };

    let mut bus = Bus {
        bus_number: 7,
        route_number: 1,
        driver,
        num_stops: 15,
        current_stop: 1,
        passengers: Vec::new(),
    };


    // make 200 stops to simulate a single day
    for n in 1..201 {
        bus.next_stop();
        bus.passengers_off();
        bus.passengers_on(rng);
    }
}