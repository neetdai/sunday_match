use criterion::{black_box, criterion_group, criterion_main, Criterion};
use homework::sunday_match;

fn bench_sunday_match(haytax: &str, target: &str) {
    let haytax = black_box(haytax);
    let target = black_box(target);
    sunday_match(haytax, target);
}

fn bench_std_two_way_match(haytax: &str, target: &str) {
    let haytax = black_box(haytax);
    let target = black_box(target);
    haytax.match_indices(target).next();
}

fn sunday_match_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("match_compare");
    group.bench_function("sunday_match", |b| {
        b.iter(|| {
            let source = "aabaaabaaac";
            let target = "aabaaac";
            bench_sunday_match(source, target);
        });
    });

    group.bench_function("std_match", |b| {
        b.iter(|| {
            let source = "aabaaabaaac";
            let target = "aabaaac";
            bench_std_two_way_match(source, target);
        })
    });

    group.finish();

    let mut group = c.benchmark_group("match_compare2");
    group.bench_function("sunday_match2", |b| {
        b.iter(|| {
            let source = "mississippi";
            let target = "sippia";
            bench_sunday_match(source, target);
        });
    });

    group.bench_function("std_match", |b| {
        b.iter(|| {
            let source = "mississippi";
            let target = "sippia";
            bench_std_two_way_match(source, target);
        })
    });
    group.finish();

    let mut group = c.benchmark_group("match_compare3");
    group.bench_function("sunday_match3", |b| {
        b.iter(|| {
            let source = r#"The career I have chosen is laden with opportunity yet it is fraught with heartbreak and despair and the bodies of those who have failed, were they piled one atop another, would cast a shadow down upon all the pyramids of the earth.
 Yet I will not fail, as the others, for in my hands I now hold the charts which will guide through perilous waters to shores which only yesterday seemed but a dream.
 Failure no longer will be my payment for struggle. Just as nature made no provision for my body to tolerate pain neither has it made any provision for my life to suffer failure. Failure, like pain, is alien to my life. In the past I accepted it as I accepted pain. Now I reject it and I am prepared for wisdom and principles which will guide me out of the shadows into the sunlight of wealth, position, and happiness far beyond my most extravagant dreams until even the golden apples in the Garden of Hesperides will seem no more than my just reward.
 Time teaches all things to him who lives forever but I have not the luxury of eternity. Yet within my allotted time I must practice the art of patience for nature acts never in haste. To create the olive, king of all trees, a hundred years is required. An onion plant is old in nine weeks. I have lived as an onion plant. It has not pleased me. Now I wouldst become the greatest of olive trees and, in truth, the greatest of salesman.
 And how will this be accomplished? For I have neither the knowledge nor the experience to achieve the greatness and already I have stumbled in ignorance and fallen into pools of self-pity. The answer is simple. I will commence my journey unencumbered with either the weight of unnecessary knowledge or the handicap of meaningless experience. Nature already has supplied me with knowledge and instinct far greater than any beast in the forest and the value of experience is overrated, usually by old men who nod wisely and speak stupidly.
 In truth, experience teaches thoroughly yet her course of instruction devours men's years so the value of her lessons diminishes with the time necessary to acquire her special wisdom. The end finds it wasted on dead men. Furthermore, experience is comparable to fashion; an action that proved successful today will be unworkable and impractical tomorrow.
 Only principles endure and these I now possess, for the laws that will lead me to greatness are contained in the words of these scrolls. What they will teach me is more to prevent failure than to gain success, for what is success other than a state of mind? Which two, among a thouand wise men, will define success in the same words; yet failure is always described but one way. Failure is man's inability to reach his goals in life, whatever they may be.
 In truth, the only difference between those who have failed and those who have successed lies in the difference of their habits. Good habits are the key to all success. Bad habits are the unlocked door to failure. Thus, the first law I will obey, which precedeth all others is --I will form good habits and become their slave.
 As a child I was slave to my impulses; now I am slave to my habits, as are all grown men. I have surrendered my free will to the years of accumulated habits and the past deeds of my life have already marked out a path which threatens to imprison my future. My actions are ruled by appetite, passion, prejudice, greed, love, fear, environment, habit, and the worst of these tyrants is habit. Therefore, if I must be a slave to habit let me be a slave to good habits. My bad habits must be destroyed and new furrows prepared for good seed.
 And I make a solemn oath to myself that nothing will retard my new life's growth. I will lose not a day from these readings for that day cannot be retrieved nor can I substitute another for it. I must not , I will not, break this habit of daily reading from these scrolls and, in truth, the few moments spent each day on this new habit are but a small price to pay for the happiness and success that will be mine.
 As I read and re-read the words in the scrolls to follow, never will I allow the brevity of each scroll nor the simplicity of its words to cause me to treat the scroll's message lightly. Thousands of grapes are pressed to fill one jar with wine, and the grapeskin and pulp are tossed to the birds. So it is with these grapes of wisdom from the ages. Much has been filtered and tossed to the wind.Only the pure truth lies distilled in the words to come. I will drink as instructed and spill not a drop. And the seed of success I will swallow."#;
            let target = "scroll nor the simplicity of its words";
            bench_sunday_match(source, target);
        });
    });

    group.bench_function("std_match", |b| {
        b.iter(|| {
            let source = r#"The career I have chosen is laden with opportunity yet it is fraught with heartbreak and despair and the bodies of those who have failed, were they piled one atop another, would cast a shadow down upon all the pyramids of the earth.
 Yet I will not fail, as the others, for in my hands I now hold the charts which will guide through perilous waters to shores which only yesterday seemed but a dream.
 Failure no longer will be my payment for struggle. Just as nature made no provision for my body to tolerate pain neither has it made any provision for my life to suffer failure. Failure, like pain, is alien to my life. In the past I accepted it as I accepted pain. Now I reject it and I am prepared for wisdom and principles which will guide me out of the shadows into the sunlight of wealth, position, and happiness far beyond my most extravagant dreams until even the golden apples in the Garden of Hesperides will seem no more than my just reward.
 Time teaches all things to him who lives forever but I have not the luxury of eternity. Yet within my allotted time I must practice the art of patience for nature acts never in haste. To create the olive, king of all trees, a hundred years is required. An onion plant is old in nine weeks. I have lived as an onion plant. It has not pleased me. Now I wouldst become the greatest of olive trees and, in truth, the greatest of salesman.
 And how will this be accomplished? For I have neither the knowledge nor the experience to achieve the greatness and already I have stumbled in ignorance and fallen into pools of self-pity. The answer is simple. I will commence my journey unencumbered with either the weight of unnecessary knowledge or the handicap of meaningless experience. Nature already has supplied me with knowledge and instinct far greater than any beast in the forest and the value of experience is overrated, usually by old men who nod wisely and speak stupidly.
 In truth, experience teaches thoroughly yet her course of instruction devours men's years so the value of her lessons diminishes with the time necessary to acquire her special wisdom. The end finds it wasted on dead men. Furthermore, experience is comparable to fashion; an action that proved successful today will be unworkable and impractical tomorrow.
 Only principles endure and these I now possess, for the laws that will lead me to greatness are contained in the words of these scrolls. What they will teach me is more to prevent failure than to gain success, for what is success other than a state of mind? Which two, among a thouand wise men, will define success in the same words; yet failure is always described but one way. Failure is man's inability to reach his goals in life, whatever they may be.
 In truth, the only difference between those who have failed and those who have successed lies in the difference of their habits. Good habits are the key to all success. Bad habits are the unlocked door to failure. Thus, the first law I will obey, which precedeth all others is --I will form good habits and become their slave.
 As a child I was slave to my impulses; now I am slave to my habits, as are all grown men. I have surrendered my free will to the years of accumulated habits and the past deeds of my life have already marked out a path which threatens to imprison my future. My actions are ruled by appetite, passion, prejudice, greed, love, fear, environment, habit, and the worst of these tyrants is habit. Therefore, if I must be a slave to habit let me be a slave to good habits. My bad habits must be destroyed and new furrows prepared for good seed.
 And I make a solemn oath to myself that nothing will retard my new life's growth. I will lose not a day from these readings for that day cannot be retrieved nor can I substitute another for it. I must not , I will not, break this habit of daily reading from these scrolls and, in truth, the few moments spent each day on this new habit are but a small price to pay for the happiness and success that will be mine.
 As I read and re-read the words in the scrolls to follow, never will I allow the brevity of each scroll nor the simplicity of its words to cause me to treat the scroll's message lightly. Thousands of grapes are pressed to fill one jar with wine, and the grapeskin and pulp are tossed to the birds. So it is with these grapes of wisdom from the ages. Much has been filtered and tossed to the wind.Only the pure truth lies distilled in the words to come. I will drink as instructed and spill not a drop. And the seed of success I will swallow."#;
            let target = "scroll nor the simplicity of its words";
            bench_std_two_way_match(source, target);
        })
    });
    group.finish();
}

criterion_group!(benches, sunday_match_bench);
criterion_main!(benches);