enum FontStyle{

 Normal,
 Italic,
 Oblique(deg : i32),

}

enum FontWeight{

  Normal,
  Bold,
  Lighter,
  Bolder,
  Boldness(val: i32),

}

enum TextAlign{

  Left,
  Right,
  Center,
  Justify,
}

enum BlockOverflow{

   Visible,
   Hidden,
   Clip,
   Scroll,
   Auto,
}

enum BlockOpacity{

   opacity(val: f32),


}

/// provides basic CSS Styling property
struct Style{

   Color : Option<String>,
   Font  : Option<String>,
   Size: Option<String>,
   Decoration: Option<String>,
   Weight: FontWeight,
   Style: FontStyle,
   Align: TextAlign,
   Bg: Option<String>,
   Height: Option<String>,
   Width : Option<String>,
   Display: Option<String>,
   Opacity: BlockOpacity,
   Border:  Option<String>,
   Padding: Option<String>,
   Overflow: BlockOverflow,

}

