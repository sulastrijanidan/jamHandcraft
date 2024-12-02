use ic_cdk::api::{call, trap};
use ic_cdk::export::candid::CandidType;
use ic_cdk::export::Principal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct Product {
    id: u64,
    name: String,
    description: String,
    price: u64,  // Harga dalam unit terkecil (misalnya, sen atau ribu)
    quantity: u64,  // Jumlah produk yang tersedia
    is_handcrafted: bool, // Menandakan apakah produk adalah jam handmade
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
struct Transaction {
    buyer: Principal,
    product_id: u64,
    quantity: u64,
    total_price: u64,
}

struct Shop {
    products: HashMap<u64, Product>,
    transactions: Vec<Transaction>,
    next_product_id: u64,
}

impl Shop {
    fn new() -> Self {
        Shop {
            products: HashMap::new(),
            transactions: Vec::new(),
            next_product_id: 1,
        }
    }

    fn add_product(&mut self, name: String, description: String, price: u64, quantity: u64, is_handcrafted: bool) -> u64 {
        let product = Product {
            id: self.next_product_id,
            name,
            description,
            price,
            quantity,
            is_handcrafted,
        };
        self.products.insert(self.next_product_id, product);
        self.next_product_id += 1;
        self.next_product_id - 1
    }

    fn list_products(&self) -> Vec<Product> {
        self.products.values().cloned().collect()
    }

    fn buy_product(&mut self, buyer: Principal, product_id: u64, quantity: u64) -> Result<(), String> {
        let product = self.products.get_mut(&product_id).ok_or("Produk tidak ditemukan")?;

        if product.quantity < quantity {
            return Err("Stok tidak cukup".to_string());
        }

        product.quantity -= quantity;
        let total_price = product.price * quantity;

        let transaction = Transaction {
            buyer,
            product_id,
            quantity,
            total_price,
        };
        self.transactions.push(transaction);

        Ok(())
    }
}

#[ic_cdk::query]
fn list_products() -> Vec<Product> {
    let shop = get_shop();
    shop.list_products()
}

#[ic_cdk::update]
fn add_product(name: String, description: String, price: u64, quantity: u64, is_handcrafted: bool) -> u64 {
    let mut shop = get_shop();
    shop.add_product(name, description, price, quantity, is_handcrafted)
}

#[ic_cdk::update]
fn buy_product(product_id: u64, quantity: u64) -> Result<(), String> {
    let buyer = ic_cdk::caller();
    let mut shop = get_shop();
    shop.buy_product(buyer, product_id, quantity)
}

// Helper function to get the app state
fn get_shop() -> &'static mut Shop {
    ic_cdk::storage::get_mut::<Shop>().unwrap()
}

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    ic_cdk::storage::stable_save((get_shop(),)).unwrap();
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (shop,): (Shop,) = ic_cdk::storage::stable_restore().unwrap();
    ic_cdk::storage::set(shop);
}
