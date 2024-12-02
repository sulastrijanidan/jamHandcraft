use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

struct Model {
    products: Vec<Product>,
    new_name: String,
    new_description: String,
    new_price: u64,
    new_quantity: u64,
    new_quantity_to_buy: u64,
    buy_product_result: Option<String>,
    link: ComponentLink<Self>,
}

#[derive(Clone, Debug)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub quantity: u64,
    pub is_handcrafted: bool,
}

enum Msg {
    AddProduct,
    BuyProduct,
    FetchProducts,
    ReceiveProducts(Vec<Product>),
    SetNewName(String),
    SetNewDescription(String),
    SetNewPrice(u64),
    SetNewQuantity(u64),
    SetNewQuantityToBuy(u64),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            products: Vec::new(),
            new_name: String::new(),
            new_description: String::new(),
            new_price: 0,
            new_quantity: 0,
            new_quantity_to_buy: 0,
            buy_product_result: None,
            link: ctx.link().clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddProduct => {
                let name = self.new_name.clone();
                let description = self.new_description.clone();
                let price = self.new_price;
                let quantity = self.new_quantity;
                let is_handcrafted = false; // Menentukan apakah produk handcrafted atau bukan
                self.link
                    .send_future(async move {
                        // Kirim request untuk menambah produk ke backend
                        let _response = add_product_to_backend(name, description, price, quantity, is_handcrafted).await;
                    });
                false
            }
            Msg::BuyProduct => {
                let product_id = 1; // Contoh, ganti dengan id produk yang dipilih
                let quantity = self.new_quantity_to_buy;
                self.link.send_future(async move {
                    // Kirim request untuk membeli produk
                    let result = buy_product_from_backend(product_id, quantity).await;
                    self.buy_product_result = Some(result);
                });
                false
            }
            Msg::FetchProducts => {
                self.link.send_future(async move {
                    // Ambil produk dari backend
                    let products = fetch_products_from_backend().await;
                    ctx.link().send_message(Msg::ReceiveProducts(products));
                });
                true
            }
            Msg::ReceiveProducts(products) => {
                self.products = products;
                true
            }
            Msg::SetNewName(name) => {
                self.new_name = name;
                true
            }
            Msg::SetNewDescription(description) => {
                self.new_description = description;
                true
            }
            Msg::SetNewPrice(price) => {
                self.new_price = price;
                true
            }
            Msg::SetNewQuantity(quantity) => {
                self.new_quantity = quantity;
                true
            }
            Msg::SetNewQuantityToBuy(quantity) => {
                self.new_quantity_to_buy = quantity;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Toko Jam dan Jam Handcrafted" }</h1>
                <button onclick={ctx.link().callback(|_| Msg::FetchProducts)}>{ "Tampilkan Produk" }</button>

                <div>
                    <h3>{ "Tambah Produk Baru" }</h3>
                    <input type="text" placeholder="Nama Produk" oninput={ctx.link().callback(|e: InputData| Msg::SetNewName(e.value))} />
                    <textarea placeholder="Deskripsi Produk" oninput={ctx.link().callback(|e: InputData| Msg::SetNewDescription(e.value))}></textarea>
                    <input type="number" placeholder="Harga" oninput={ctx.link().callback(|e: InputData| Msg::SetNewPrice(e.value.parse().unwrap_or(0)))}/>
                    <input type="number" placeholder="Jumlah" oninput={ctx.link().callback(|e: InputData| Msg::SetNewQuantity(e.value.parse().unwrap_or(0)))}/>
                    <button onclick={ctx.link().callback(|_| Msg::AddProduct)}>{ "Tambah Produk" }</button>
                </div>

                <div>
                    <h3>{ "Daftar Produk" }</h3>
                    <ul>
                        { for self.products.iter().map(|product| html!{ 
                            <li>{ format!("{} - {} - {} unit - Rp {}", product.name, product.quantity, product.price) }</li> 
                        }) }
                    </ul>
                </div>

                <div>
                    <h3>{ "Beli Produk" }</h3>
                    <input type="number" placeholder="Jumlah untuk dibeli" oninput={ctx.link().callback(|e: InputData| Msg::SetNewQuantityToBuy(e.value.parse().unwrap_or(0)))}/>
                    <button onclick={ctx.link().callback(|_| Msg::BuyProduct)}>{ "Beli Produk" }</button>
                </div>

                {
                    if let Some(result) = &self.buy_product_result {
                        html! { <p>{ result }</p> }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}

async fn fetch_products_from_backend() -> Vec<Product> {
    // Panggil API backend untuk mengambil daftar produk
    vec![]
}

async fn add_product_to_backend(name: String, description: String, price: u64, quantity: u64, is_handcrafted: bool) -> Result<(), String> {
    // Panggil API backend untuk menambah produk
    Ok(())
}

async fn buy_product_from_backend(product_id: u64, quantity: u64) -> String {
    // Panggil API backend untuk membeli produk
    "Pembelian berhasil!".to_string()
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
