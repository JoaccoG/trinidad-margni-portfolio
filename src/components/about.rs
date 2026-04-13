use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section
            id="about"
            class="bg-secondary text-dark py-16 md:py-24 px-4 sm:px-8 lg:px-16"
            aria-labelledby="about-heading"
        >
            <div class="mx-auto grid max-w-6xl grid-cols-1 gap-10 md:grid-cols-[minmax(0,1fr)_minmax(0,1fr)] md:gap-14 md:items-center">
                <figure class="order-1 min-w-0 w-full overflow-hidden rounded-sm md:order-none">
                    <img
                        src="/public/assets/images/about-me.png"
                        alt="Trinidad Margni"
                        class="aspect-[4/5] h-auto w-full max-h-[min(70vh,28rem)] object-cover object-top md:aspect-[3/4] md:max-h-none md:min-h-[28rem]"
                    />
                </figure>

                <div class="order-2 flex min-w-0 flex-col gap-5 text-left md:gap-6">
                    <header class="space-y-2 sm:space-y-3">
                        <p class="font-display text-xl sm:text-2xl md:text-4xl">"Hey there"</p>
                        <h2
                            id="about-heading"
                            class="font-serif text-2xl uppercase leading-tight tracking-wide text-balance sm:text-xl md:text-2xl lg:text-3xl xl:text-4xl"
                        >
                            "I'm Trinidad Margni"
                        </h2>
                    </header>

                    <p class="max-w-prose font-sans text-xs font-semibold leading-snug sm:text-sm md:text-base">
                        "Senior Project Manager leading execution across global teams within complex marketing and tech environments."
                    </p>

                    <div class="max-w-prose space-y-4 font-sans text-xs leading-relaxed text-dark/90 sm:text-sm md:text-base">
                        <p>
                            "I have led projects for organizations such as United Airlines and YouTube, and I currently support high-performing teams working with ServiceNow—ensuring alignment, consistent delivery, and continuous improvement across distributed teams."
                        </p>
                        <p>
                            "With a strong foundation in leadership and ongoing MBA studies, I specialize in transforming complexity into structured execution, fostering collaboration, and delivering impactful results."
                        </p>
                    </div>

                    <details class="about-resume relative inline-block w-fit">
                        <summary class="flex cursor-pointer list-none items-center justify-center gap-2 bg-dark px-6 py-3 font-sans text-xs tracking-[0.2em] uppercase text-light transition-opacity hover:opacity-90 [&::-webkit-details-marker]:hidden marker:content-none">
                            "MY RESUME"
                            <img
                                src="/public/assets/icons/chevron-down.svg"
                                alt=""
                                width="12"
                                height="12"
                                class="about-resume-chevron h-3 w-3 shrink-0"
                                aria-hidden="true"
                            />
                        </summary>
                        <div class="absolute left-0 top-full z-50 flex min-w-full w-max flex-col border border-dark bg-secondary/95 text-dark shadow-lg backdrop-blur-sm">
                            <a
                                class="flex w-full items-center justify-between gap-3 border-b border-dark/20 px-4 py-3 font-sans text-xs tracking-[0.15em] uppercase transition-colors hover:bg-primary"
                                href="/public/files/resume.pdf"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "English"
                                <img
                                    src="/public/assets/icons/external-link.svg"
                                    alt=""
                                    width="14"
                                    height="14"
                                    class="h-3.5 w-3.5 shrink-0"
                                    aria-hidden="true"
                                />
                            </a>
                            <a
                                class="flex w-full items-center justify-between gap-3 px-4 py-3 font-sans text-xs tracking-[0.15em] uppercase transition-colors hover:bg-primary"
                                href="/public/files/cv.pdf"
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "Spanish"
                                <img
                                    src="/public/assets/icons/external-link.svg"
                                    alt=""
                                    width="14"
                                    height="14"
                                    class="h-3.5 w-3.5 shrink-0"
                                    aria-hidden="true"
                                />
                            </a>
                        </div>
                    </details>
                </div>
            </div>
        </section>
    }
}
