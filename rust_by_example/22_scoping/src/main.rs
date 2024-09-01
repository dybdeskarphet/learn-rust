fn main() {
    let immutable_box = Box::new(29u32);

    println!("immutable box contains {}", immutable_box);

    let mut mutable_box = immutable_box;

    *mutable_box = 20;
    println!("mutable box contains {}", mutable_box);

    let immutastring = "I'm not mutable".to_owned();
    let mut mutstring = "I'm mutable".to_owned();
    let mut mutstring2 = "I'm mutable too".to_owned();

    let mutimmutref = &mutstring;
    let mutmutref = &mut mutstring2;
    let immutaref = &immutastring;

    println!("Immutable reference to mutable string: {}", mutimmutref);
    println!("Mutable reference to mutable string: {}", mutmutref);
    println!("Immutable reference to immutable string: {}", immutaref);

    // NOTE: Also, when the immutable refernce of a mutable is no longer used, you can borrow it again mutabily.

    let c = 'C';

    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("Is ref_c1 eqaul to ref_c2? {}", *ref_c1 == *ref_c2)
    // NOTE: Altough we dereferenced them, we actually don't have to because in this case, deref coercion is applied.
}
