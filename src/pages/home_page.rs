use crate::components::header::Header;
use yew::prelude::*;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
      <>
        <Header />
        <section class="bg-ct-wv-dark min-h-screen p-8">
            <div class="max-w-4xl mx-auto bg-ct-wv-white rounded-md h-[20rem] flex justify-center items-center">
                <p class="text-3xl text-ct-wv-dark-text font-semibold">{"Welcome!!"}</p>
            </div>
        </section>
      </>
    }
}
