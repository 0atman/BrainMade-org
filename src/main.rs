use color_eyre::Report;
#[allow(clippy::wildcard_imports)]
use html_node::{
    typed::{elements::*, html},
    Node,
};

fn template(inner: Node) -> Node {
    html! {

        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta http-equiv="x-clacks-overhead" content="GNU Terry Pratchett" />
                <link rel="icon" href="favicon.png"/>
                <script src="https://cdn.tailwindcss.com"></script>
                <script src="tw.js"></script>

            <script>
                tailwind.config = {
                    theme: {
                        container: {
                            center: true,
                        },
                        fontFamily: {
                            "mono": "courier, monospace",
                        }
                    }
                }
            </script>

            <meta charset="utf-8"/>
            <meta name="description" content="When you see this logo on any artwork, whether painting, poetry, or prose, you know that it was made by a human just like you."/>

            <meta content="width=device-width, initial-scale=1" name="viewport"/>
            <title class="text-4xl" >"The Brainmade Mark"</title>

            </head>

                <body class="bg-black text-white font-mono text-sm md:text-2xl mx-auto w-full max-w-5xl">

                    <nav class="flex items-center justify-between flex-wrap bg-black-500 p-6">
                        <div class="flex items-center flex-shrink-0 text-white mr-6">
                            <span class="font-semibold text-xl tracking-tight">The Brainmade Mark</span>
                        </div>
                        <div class="w-full block flex-grow lg:flex lg:items-center lg:w-auto">
                            <div class="text-xl lg:flex-grow">
                                <a href="index.html#about" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    About
                                </a>
                                <a href="#downloads" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Downloads
                                </a>
                                <a href="https://github.com/0atman/Brainmade-org" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                                    Github
                                </a>
                            </div>
                        </div>
                    </nav>

                    <div class="border-black border-8 container mx-auto">

                            {inner}

                    </div>

            </body>
        </html>
    }
}

/// NOTE: the widget requires https to load
fn widget() -> Node {
    html_node::html! {
        <img class="w-1/2" src="video.png" />
    }
}

#[allow(clippy::too_many_lines)]
fn index() -> Node {
    let heading = html! {
        <div class="flex w-full justify-center">
            <img class="w-1/2" alt="A logo of a human with a seed germinating in their head, with the word 'Brainmade' next to it, along with the website brainmade.org underneath." src="white-logo.svg" />
        </div>
        <br/>
        <br/>
        <h2 class="slogan"><b class="text-2xl" > "When you see this logo on any artwork, whether painting, poetry, or prose, you know that it was made by a human just like you." </b></h2>
    };

    let intro = html! {
        "The "<i>Brainmade</i>" mark is something you can attach to any works that are mostly made by you or your friends, not by generative tools like GPT. I've built this website to freely share the high-resolution black or white versions of the logo with you, which you can download and attach to your own projects if you'd like to make this statement."
        <br/>
        "I hope the following video will explain in detail for making this clear, but the tl;dr is:"
        <br/>
        <br/>
        <ul class="list-decimal">
            <li><b>"I don't hate AIs,"</b></li>
            <li><b>"I love humans!"</b></li>
        </ul>
        <br/>
        <br/>
        <b>
            <a class="underline" href="https://youtu.be/kul0z3OTmVM">Watch my short video here, or read on.
                { widget() }
            </a>
        </b>
    };

    let sect1 = html! {
        <h1 class="text-4xl" id="about"><b>About</b></h1>
        "I'm Tris, I'm a writer and producer of "<a class="underline" href="http://noboilerplate.org">"fast, technical videos"</a>", and "<a class="underline" href="https://namtao.com">"audiofiction and music."</a>
        <br/>
        "My first career was as a web developer, doing production on the side for 15 years, but in 2022 I accidentally become entirely self-employed thanks to the surprising success of my YouTube channel, No Boilerplate."
        <br/>
        <br/>
        "At heart I'm still a software developer, I'll re-use 100 libraries to avoid writing 10 lines of code - standing on the shoulders of giants is the only way I know how I get around."
        <br/>
        "But I've looked for a way to mark my videos and stories as being made by humans, not AI, and I can't find one that works in exactly the way I want."
        <br/>
        "I don't want something that says 'NO AI USED', signposts that are negative and judgemental, nor a '100% human made' guarantee - what would that even MEAN these days?"
        <br/>
        "I want a positive mark."
        <br/>
        <br/>
        "I have many issues with the options I've seen so far, from having multiple logos (which is confusing) to the fixation on AI being inherently evil (this will not always be the case)."
        <br/>
        "My root concern with these methods is that they are negative. `AI = bad`.
        But I think the correct way to present this is `human = good`."
        <br/>
        <br/>
        "I don't need 100% human made, I perhaps need 90% human made. Three example may make my thinking clearer:"
        <ul class="list-decimal">
            <li>
                "Using, say, chatgpt as a rhyming dictionary feels fine, but writing whole verses of your poem doesn't."
            </li>
            <li>
                "Using DALL-E to start brainstorming with 100 generated views of birds sitting on telephone lines seems fine, but getting it to paint large sections of your artwork doesn't."
            </li>
            <li>
                "Asking a text generator to give you 10 happy-sounding synonyms for despair sparks joy in me, but asking it to write your anti-trancendentalist masterpiece does not."
            </li>
        </ul>
        <br/>
        "Using these tools to make more of the artwork you want is valid, but you're not a creator, you're still a consumer. I'm not sure exactly what 'too much AI' is, but just like your audience, I'll know it when I see it."
        <br/>
        <br/>
        "I love knowing a human made the artwork I'm consuming."
        <br/>
        "There's "<i>something</i>" there, something transcendent and magical."
        <br/>
        <br/>
        "I "<i>like</i>" that you tried hard, that's part of the experience."
    };

    let sect2 = html! {
        <h2 id="downloads" class="text-2xl"><b>Downloads</b></h2>

        <h3 class="text-l"><b>White</b></h3>
        <ul class="list-disc">
            <li>
                <a href="white-logo.png" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                    white-logo.png
                </a>
            </li>
            <li>
                <a href="white-logo.svg" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                    white-logo.svg
                </a>
            </li>
        </ul>

        <h3 class="text-l"><b>Black</b></h3>
        <ul class="list-disc">
            <li>
                <a href="black-logo.png" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                    black-logo.png
                </a>
            </li>
            <li>
                <a href="black-logo.svg" class="underline block lg:inline-block lg:mt-0 text-black-200 hover:text-white mr-4">
                    black-logo.svg
                </a>
            </li>
        </ul>

    };

    let credits = html! {
        <h2 class="text-2xl" > <b>Credits</b> </h2>
        <ul class="list-disc">
            <li>
                "Logo based on \"Human\" by JunGSa from "
                <a class="underline" href="https://thenounproject.com/browse/icons/term/human/" target="_blank" title="Human Icons">Noun Project</a>
            </li>
            <li>
                "And \"seed\" by Adrian Syauqi from "
            <a class="underline" href="https://thenounproject.com/browse/icons/term/seed/" target="_blank" title="seed Icons">Noun Project</a>
            </li>
        </ul>
        <br/>
        "Special thanks to everyone who workshopped the logo with me, especially super patron supporter Andrew Jackson. Andrew, I should be paying YOU!"
    };

    let footer = html! {
        <br/>
        <br/>
        <br/>
        <br/>
        <p class="text-xs">"Brainmade is a NAMTAO production, made with <3 in 2024"</p>
    };

    page(heading, intro, sect1, sect2, credits, footer)
}

fn page(heading: Node, intro: Node, sect1: Node, sect2: Node, credits: Node, footer: Node) -> Node {
    template(html! {
        { heading }
        <br/>
        { intro }
        <br/>
        <br/>
        { sect1 }
        <br/>
        <br/>
        { sect2 }
        <br/>
        <br/>
        { credits }
        <br/>
        <br/>
        { footer }
    })
}

type Router<'a> = Vec<(&'a str, fn() -> Node)>;

fn build(pages: Router) -> Result<(), Report> {
    for (page, fun) in pages {
        std::fs::write(page, fun().to_string())?;
    }
    Ok(())
}

fn main() -> Result<(), Report> {
    std::fs::create_dir_all("docs")?;
    let _ = build(vec![("docs/index.html", index)]);
    println!("Built site OK!");
    Ok(())
}
