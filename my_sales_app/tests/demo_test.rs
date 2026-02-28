use sales::{
    Encoder, ReadBuf, WriteBuf,
    message_header_codec::{self, MessageHeaderDecoder},
    order_message_codec::{OrderMessageDecoder, OrderMessageEncoder, encoder::ItemsEncoder},
};

#[test]
fn simple() {
    // ****** BUFFER ******
    let mut buffer = [0u8; 512];

    // ****** ENCODE *******
    let mut order = OrderMessageEncoder::default().wrap(
        WriteBuf::new(&mut buffer),
        message_header_codec::ENCODED_LENGTH,
    );
    // short cut creation of header with block length, version, schema, template id auto set
    order = order.header(0).parent().unwrap();

    order.order_id(234);
    order.client_id(135);
    order.timestamp(246);
    order.order_type(sales::order_type::OrderType::New);

    let mut items = order.items_encoder(2, ItemsEncoder::default());

    let _r = items.advance();
    items.product_id(222);
    items.quantity(2);
    let mut price = items.unit_price_encoder();
    price.mantissa(2345); // exponent is -2 (two decimals) constant in xml schema
    items = price.parent().unwrap();

    let _r = items.advance();
    items.product_id(333);
    items.quantity(3);
    let mut price = items.unit_price_encoder();
    price.mantissa(4567); // exponent is -2 (two decimals) constant in xml schema
    items = price.parent().unwrap();

    order = items.parent().unwrap();

    order.customer_note("duck is angry");

    let encoded_len = order.get_limit() as usize;
    let dbg_buffer = &buffer[..encoded_len];
    println!("{:?}", dbg_buffer);

    // ****** DECODE ******
    let mut header = MessageHeaderDecoder::default();
    let read_buf = ReadBuf::new(&buffer);
    header = header.wrap(read_buf, 0);

    let block_length = header.block_length();
    let version = header.version();
    let template_id = header.template_id(); // from tmpl id we know what type of message it is

    println!("template_id = {template_id}, block_length = {block_length}, version = {version}");

    let mut order = OrderMessageDecoder::default();
    order = order.wrap(
        read_buf,
        message_header_codec::ENCODED_LENGTH,
        block_length,
        version,
    );

    println!(
        "order_id = {}, client_id = {}, timestamp = {} order_type = {}",
        order.order_id(),
        order.client_id(),
        order.timestamp(),
        order.order_type()
    );

    let mut items = order.items_decoder();

    let count = items.count();

    for i in 1..=count {
        let _r = items.advance();
        let product_id = items.product_id();
        let qty = items.quantity();
        let mut price = items.unit_price_decoder();
        let exponent = price.exponent();
        let mantissa = price.mantissa();
        println!("{i}. product_id = {product_id}, qty = {qty}, price = {mantissa} x 10^{exponent}");
        items = price.parent().unwrap();
    }
    order = items.parent().unwrap();

    let (offset, len) = order.customer_note_decoder();
    let bytes = &buffer[offset..offset + len];
    let note = std::str::from_utf8(bytes).unwrap();
    println!("customer_note = {:?}", note);
}
