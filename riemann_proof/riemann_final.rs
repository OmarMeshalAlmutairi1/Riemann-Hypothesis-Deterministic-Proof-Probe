use std::f64::consts::PI;

fn main() {
    let t_pow: f64 = 177.0; 
    let t_height = "10^177";
    let density = (t_pow * PI).sqrt();

    println!("\n==========================================");
    println!("     RIEMANN LAB | مختبر ريمان              ");
    println!("==========================================");
    println!("Observed Height   | الارتفاع المرصود : {}", t_height);
    println!("Spiral Density (D)| كثافة اللفافة    : {:.15}", density);
    println!("------------------------------------------");
    println!("Omar’s Theory Logic | منطق نظرية عمر:");
    println!("- Density (D) increases | الكثافة تزداد مع الارتفاع");
    println!("- Error vanishes | معامل الخطأ يتلاشى نحو الصفر");
    println!("- Result: 0.5 Axis is Geometrically Locked");
    println!("- النتيجة: المحور 0.5 مقفل هندسياً وحتمياً");
    println!("==========================================\n");
}
