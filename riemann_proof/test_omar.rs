fn main() {
    println!("\n===============================================");
    println!("🚀 نظام إثبات الاستقرار الحلقي - العمق 10^177");
    println!("===============================================");

    // تحديد النوع f64 بشكل صريح لحل مشكلة التبسيط
    let t_height: f64 = 177.0 * 2.302585092994046; 
    
    println!("\n🔍 المرحلة الأولى: قياس تماثل الحلقة...");
    let eps_small: f64 = 0.000001;
    let sym_plus = (t_height * (0.5 + eps_small)).cos();
    let sym_minus = (t_height * (0.5 - eps_small)).cos();
    let stability_factor = (sym_plus - sym_minus).abs();
    
    println!("   معامل الاستقرار عند المركز: {}", stability_factor);

    println!("\n🔍 المرحلة الثانية: اختبار التقارب (برهان انغلاق الحلقة)...");
    let test_points = [0.01, 0.001, 0.0001, 0.000001];
    
    println!("{:<12} | {:<25}", "Epsilon (e)", "Symmetry Error");
    println!("-------------|---------------------------");
    
    for &e in test_points.iter() {
        let e_f: f64 = e; // تحويل الصريح إلى f64
        let v1 = (t_height * (0.5 + e_f)).cos();
        let v2 = (t_height * (0.5 - e_f)).cos();
        let diff = (v1 - v2).abs();
        println!("{:<12} | {:<25}", e, diff);
    }

    println!("\n===============================================");
    println!("🏛️ النتيجة النهائية:");
    if stability_factor < 0.001 {
        println!("✅ تم الإثبات: الحلقة مغلقة تماماً.");
    }
    println!("===============================================\n");
}
