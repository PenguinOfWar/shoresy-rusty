// these are libraries like npm packages
use rand::seq::SliceRandom;
use rand::thread_rng;
use structopt::StructOpt;

// i don"t really understand this but i think its extending Clifrom StructOpt
#[derive(Debug, StructOpt)]
struct Cli {
    // Then name of the person shoresy is going to ball bust passed as an argument
    // We use this weird looking syntax to set defaults on our structopt
    #[structopt(default_value = "loser")]
    name: String,
}

// Configure our shoresy target types

struct Target {
    name: String,
}

fn main() {
    let args = Cli::from_args();

    // get the name from args
    // notice the & in front of the variable here
    let name = &args.name;

    // let us create our target
    // we'll do it with a struct to learn the basics

    let target = Target {
        name: name.to_string(), // it really wanted me to add to_string() to this to be extra safe,
    };

    // sometimes shoresy scores some collateral damage

    let collateral = Target {
        name: "Sam".to_string(),
    };

    // arrays are static in rust so this should be interesting
    // set up our fuckResponses as a vector in rust
    // if we don't manipulating them both gets weird

    let fuck_responses = vec![
        "Fuck you {target} your mom keeps tryin' to slip a finger in my bum but I keep telling her that I only let {collateral}'s mom do that ya fuckin loser",
        "Fuck you {target}, I made your mom cum so hard that they made a canadian heritage moment out of it and Don Mckellar played my dick",
        "Fuck you {target} your mom shot cum straight across the room and killed my siamese fighting fish, threw off the PH levels in my aquarium",
        "Fuck you {target}. Your breath's so bad it gave me an existential crisis it made me question my whole life",
        "Fuck you {target}. Your mom loves butt play the way I love Haagen daz. Let's go get some fucking ice cream",
        "Fuck you {target}, fight me see what happens",
        "Fuck you {target}, tell your mom I drained the bank account she set up for me. Top it up so I can get some fucking KFC"
      ];

    // set up our normal responses too

    let responses = vec![
        "Give your balls a tug, titfucker",
        "Your mom liked my Instagram post from two years ago from Puerto Vallarta. Tell her I'll put my swim trunks on anytime she likes",
        "Shoulda heard your mom last night she sounded like a window closing on a Tonkinese cats tail like, mmmrrrooowwwwwowowwww",
        "Shoulda heard your mom last night she sounded like my great aunt when I pop in for a surprise visit like, ooooooooooohhhhhhhhhhhhh",
        "I made an oopsies, can you tell your mom to pick up {collateral}'s mom on the way over to my place I double booked them by mistake you fuckin loser",
        "I made your mom so wet Trudeau had to deploy a 24 hour national guard unit to stack sandbags around my bed",
        "Your life's so sad I get a charity tax break just for hanging out with you. Let's get some fuckin' gyozas",
        "Your life's so sad I get a charity tax break just for hanging out with you",
        "Nice sweep. No sweep. Give your balls a tug"
      ];

    // concatenate our arrays
    let concatenated = vec![fuck_responses, responses].concat();

    // pray to rngesus for his blessing

    let mut rng = thread_rng();

    // there's a lot going on here but we take a random sample from the concat vec
    // it returns in the format (Some(yourstring)) so we need to .unwrap() it twice (?)
    // then we can run the two replaces in place

    let random_line = concatenated
        .choose(&mut rng)
        .unwrap()
        .to_string()
        // so it turns out that rust or terminal doesn't like the apostrophe
        // in literal `'` or unicode `\u{0027}`
        // so we'll use Curly instead at `\u{2019}` and just replace it
        .replace("\'", "\u{2019}")
        .replace("{collateral}", &collateral.name)
        .replace("{target}", &target.name);

    // there's probably a better way of doing this but we're not gonna sweat it

    println!("{:?}", r#random_line);
}
