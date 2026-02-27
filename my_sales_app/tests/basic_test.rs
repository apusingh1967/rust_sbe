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

    let mut header = order.header(0);
    order = header.parent().unwrap();

    order.order_id(234);
    order.client_id(135);
    order.timestamp(246);
    order.order_type(sales::order_type::OrderType::Cancel);

    let mut items = order.items_encoder(1, ItemsEncoder::default());

    let _r = items.advance();
    items.product_id(222);
    items.quantity(2);
    let mut price = items.unit_price_encoder();
    price.mantissa(22);
    items = price.parent().unwrap();

    order = items.parent().unwrap();

    order.customer_note("duck is angry");

    let encoded_len = order.get_limit() as usize;
    let buffer2 = &buffer[..encoded_len];
    println!("{:?}", buffer2);

    // ****** DECODE ******
    let mut header = MessageHeaderDecoder::default();
    let read_buf = ReadBuf::new(&buffer);
    header = header.wrap(read_buf, 0);

    let block_length = header.block_length();
    let version = header.version();
    let template_id = header.template_id();

    let mut order = OrderMessageDecoder::default();
    order = order.wrap(
        read_buf,
        message_header_codec::ENCODED_LENGTH,
        block_length,
        version,
    );

    println!("template_id! = {}", template_id);
    println!("order_id = {}", order.order_id());
    println!("client_id = {}", order.client_id());
    println!("timestamp = {}", order.timestamp());
    println!("order_type = {:?}", order.order_type());

    let mut items = order.items_decoder();

    let _r = items.advance();
    println!("product_id = {}", items.product_id());
    println!("qty = {}", items.quantity());
    let mut price = items.unit_price_decoder();
    println!("price = {}", price.mantissa());
    items = price.parent().unwrap();

    order = items.parent().unwrap();

    let (offset, len) = order.customer_note_decoder();
    let bytes = &buffer[offset..offset + len];
    let note = std::str::from_utf8(bytes).unwrap();
    println!("customer_note = {:?}", note);
}
