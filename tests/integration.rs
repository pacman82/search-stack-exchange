use search_stack_exchange::PostReader;
use quick_xml::{events::Event};

#[test]
fn find_similar_question() {
    // Parse Post.xml from 3dprinting stackexchange dump. We choose the 3d printing dump, because it
    // is one of the smaller ones.
    let mut reader = PostReader::new("./tests/3dprinting.Posts.xml").unwrap();
    // Self start row event
    let row_event = reader.xml_reader.read_event_into(&mut reader.buf).unwrap();
    // let row_event = reader.read_event_into(&mut buf).unwrap();
    match row_event {
        Event::Empty(e) => {
            assert_eq!(e.name().as_ref(), b"row");
            let mut id = None;
            let mut post_type_id = None;
            for attr in e.attributes() {
                println!("{attr:?}");
                let attr = attr.unwrap();
                match attr.key.into_inner() {
                    b"Id" => id = Some(attr.value),
                    b"PostTypeId" => post_type_id = Some(attr.value),
                    _ => (),
                }
            }
            assert_eq!(id.as_deref(), Some(b"1".as_slice()));
            assert_eq!(post_type_id.as_deref(), Some(b"1".as_slice()));
        }
        // Event::Text(e) => {

        // }
        any => panic!("Expected empty event. Found {any:?}"),
    }
}
