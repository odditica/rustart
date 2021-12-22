extern crate unicode_segmentation;
use rand::Rng;
use unicode_segmentation::UnicodeSegmentation;

const WORD_LIST: &[&str] = &[    
"🍇 Grapes",
"🍈 Melon",
"🍉 Watermelon",
"🍊 Tangerine",
"🍋 Lemon",
"🍌 Banana",
"🍍 Pineapple",
"🥭 Mango",
"🍎 Red Apple",
"🍏 Green Apple",
"🍐 Pear",
"🍑 Peach",
"🍒 Cherries",
"🍓 Strawberry",
"🥝 Kiwi Fruit",
"🍅 Tomato",
"🥥 Coconut",
"🥑 Avocado",
"🍆 Eggplant",
"🥔 Potato",
"🥕 Carrot",
"🌽 Corn",
"🌶️ Hot Pepper",
"🥒 Cucumber",
"🥬 Leafy Green",
"🥦 Broccoli",
"🧄 Garlic",
"🧅 Onion",
"🍄 Mushroom",
"🥜 Peanuts",
"🌰 Chestnut",
"🍞 Bread",
"🥐 Croissant",
"🥖 Baguette Bread",
"🥨 Pretzel",
"🥯 Bagel",
"🥞 Pancakes",
"🧇 Waffle",
"🧀 Cheese",
"🍖 Meat on Bone",
"🥩 Cut of Meat",
"🥓 Bacon",
"🍔 Hamburger",
"🍟 French Fries",
"🍕 Pizza",
"🌭 Hot Dog",
"🥪 Sandwich",
"🌮 Taco",
"🌯 Burrito",
"🧆 Falafel",
"🥚 Egg",
"🍳 Cooking",
"🥗 Green Salad",
"🍿 Popcorn",
"🧈 Butter",
"🧂 Salt",
"🥫 Canned Food",
"🍱 Bento Box",
"🍘 Rice Cracker",
"🍙 Rice Ball",
"🍚 Cooked Rice",
"🍛 Curry Rice",
"🍝 Spaghetti",
"🍢 Oden",
"🍣 Sushi",
"🍤 Fried Shrimp",
"🥮 Moon Cake",
"🍡 Dango",
"🥟 Dumpling",
"🥠 Fortune Cookie",
"🦪 Oyster",
"🍦 Soft Ice Cream",
"🍧 Shaved Ice",
"🍩 Doughnut",
"🍪 Cookie",
"🎂 Birthday Cake",
"🍰 Shortcake",
"🧁 Cupcake",
"🥧 Pie",
"🍫 Chocolate Bar",
"🍬 Candy",
"🍭 Lollipop",
"🍮 Custard",
"🍯 Honey Pot",
"☕ Coffee",
"🧃 Beverage Box",
"🧉 Mate",
"🧊 Ice",
"🥢 Chopsticks",
"🍴 Fork and Knife",
"🥄 Spoon",
];

pub fn get_random_word() -> Vec<Option<String>> {
    let index = rand::thread_rng().gen_range(0..WORD_LIST.len());
    let chosen_word: String = WORD_LIST[index].to_uppercase();
    let graphemes = UnicodeSegmentation::graphemes(&chosen_word[..], true).collect::<Vec<&str>>();
    assert_ne!(graphemes.is_empty(), true);

    // Separate into graphemes, replacing whitespace with None
    return graphemes
        .iter()
        .map(|x| {
            if x.trim().is_empty() {
                None
            } else {
                Some(x.to_string())
            }
        })
        .collect();
}
