// enum_methods.rs

enum PaymentMode
{
    Debit,
    Credit,
    Paypal
}

// Bunch of dummy payment handlers

fn pay_by_credit(amt: u64)
{
    println!("Processing credit card payment of {}.", amt);
}

fn pay_by_debit(amt: u64)
{
    println!("Processing debit card payment of {}.", amt);
}

fn paypal_redirect(amt: u64)
{
    println!("Redirecting to paypal for amount of {}.", amt);
}

impl PaymentMode
{
    fn pay(&self, amount: u64)
    {
        match self
        {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount)
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode
{
    PaymentMode::Debit
}

fn main()
{
    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512);
}