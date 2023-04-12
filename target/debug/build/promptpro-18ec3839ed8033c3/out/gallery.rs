# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] mod slint_generatedApp {
     use slint :: private_unstable_api :: re_exports :: * ;
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerGeneratePageAdapter {
         r#key_words : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedString > , r#prompts : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedString > , r#generate : slint :: private_unstable_api :: re_exports :: Callback < () , () > , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , }
     impl InnerGeneratePageAdapter {
         fn new () -> :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> {
             slint :: private_unstable_api :: re_exports :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#key_words }
             . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#prompts }
             . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             }
         }
     pub struct r#GeneratePageAdapter < 'a > (& 'a :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < InnerGeneratePageAdapter >>) ;
     impl < 'a > r#GeneratePageAdapter < 'a > {
         # [allow (dead_code)] pub fn invoke_generate (& self ,) -> () {
             let _self = self . 0 . as_ref () ;
             {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#generate }
             . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_generate (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = self . 0 . as_ref () ;
             # [allow (unused)] {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#generate }
             . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_key_words (& self) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#key_words }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_key_words (& self , value : slint :: private_unstable_api :: re_exports :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#key_words }
             . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_prompts (& self) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#prompts }
             . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_prompts (& self , value : slint :: private_unstable_api :: re_exports :: SharedString) {
             # [allow (unused_imports)] let _self = self . 0 . as_ref () ;
             {
                 * & InnerGeneratePageAdapter :: FIELD_OFFSETS . r#prompts }
             . apply_pin (_self) . set (value as _) }
         }
     impl < 'a > slint :: Global < 'a , r#App > for r#GeneratePageAdapter < 'a > {
         fn get (component : & 'a r#App) -> Self {
             Self (& component . 0 . globals . global_GeneratePageAdapter) }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_98 {
         r#dark_color_scheme : slint :: private_unstable_api :: re_exports :: Property < bool > , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , }
     impl InnerColorSchemeSelector_98 {
         fn new () -> :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> {
             slint :: private_unstable_api :: re_exports :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerPalette_99 {
         r#dark_color_scheme : slint :: private_unstable_api :: re_exports :: Property < bool > , r#neutralDark : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: Color > , r#neutralLighter : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: Color > , r#neutralPrimary : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: Color > , r#neutralTertiary : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: Color > , r#white : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: Color > , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , }
     impl InnerPalette_99 {
         fn new () -> :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> {
             slint :: private_unstable_api :: re_exports :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_99 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280295198f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294243572f64 as u32)) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_99 :: FIELD_OFFSETS . r#neutralLighter }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_99 :: FIELD_OFFSETS . r#neutralPrimary }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281479472f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_99 :: FIELD_OFFSETS . r#neutralTertiary }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerPalette_99 :: FIELD_OFFSETS . r#white }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280229663f64 as u32)) as _ }
                    ) as _ }
                ) ;
                 }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_100 {
         root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , }
     impl InnerStyleMetrics_100 {
         fn new () -> :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> {
             slint :: private_unstable_api :: re_exports :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < Self >> , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) {
             # ! [allow (unused)] self . root . set (VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerSideBarItem_root_74 {
         r#root_74 : slint :: private_unstable_api :: re_exports :: r#Empty , r#state_opacity_75 : slint :: private_unstable_api :: re_exports :: r#Opacity , r#state_76 : slint :: private_unstable_api :: re_exports :: r#Rectangle , r#l_77 : slint :: private_unstable_api :: re_exports :: r#Empty , r#label_78 : slint :: private_unstable_api :: re_exports :: r#Text , r#touch_79 : slint :: private_unstable_api :: re_exports :: r#TouchArea , r#root_74_has_focus : slint :: private_unstable_api :: re_exports :: Property < bool > , r#root_74_l_77_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#root_74_l_77_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_74_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_74_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_74_selected : slint :: private_unstable_api :: re_exports :: Property < bool > , r#root_74_state : slint :: private_unstable_api :: re_exports :: Property < i32 > , r#root_74_touch_79_horizontal_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , r#root_74_touch_79_max_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_74_touch_79_max_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_74_touch_79_min_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_74_touch_79_min_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_74_touch_79_preferred_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_74_touch_79_preferred_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_74_touch_79_vertical_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerSideBarItem_root_74 >> , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerSideBarItem_root_74 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_l_77_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_l_77_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                     + r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                         + r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (if ({
                                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_selected }
                            ) . apply_pin (_self) . get () {
                                 3f64 }
                             else {
                                 (if ({
                                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_has_focus }
                                ) . apply_pin (_self) . get () {
                                     4f64 }
                                 else {
                                     (0f64) as _ }
                                ) as _ }
                            ) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_touch_79_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_opacity_75 }
                 + r#Opacity :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_opacity_75 }
                 + r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_74_state = ({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_root_74_state . clone () as f64) == (1f64 as f64)) {
                             0.8f64 }
                         else {
                             (if ((r#tmp_root_74_state . clone () as f64) == (2f64 as f64)) {
                                 0.6f64 }
                             else {
                                 (if ((r#tmp_root_74_state . clone () as f64) == (3f64 as f64)) {
                                     1f64 }
                                 else {
                                     (if ((r#tmp_root_74_state . clone () as f64) == (4f64 as f64)) {
                                         0.8f64 }
                                     else {
                                         (0f64) as _ }
                                    ) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = r#PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = slint :: private_unstable_api :: re_exports :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_opacity_75 }
                 + r#Opacity :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_76 }
                 + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#white . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_76 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_76 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#l_77 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#l_77 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#l_77 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_l_77_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                         + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (r#tmp_l_77_padding . clone () as f64)) as f64) - (r#tmp_l_77_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_l_77_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                 + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_l_77_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                 + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_opacity_75 }
             + r#Opacity :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_opacity_75 }
             + r#Opacity :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_76 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_76 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#l_77 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (({
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_l_77_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#preferred as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 5usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (5usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => Default :: default () , }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerButton_root_80 {
         r#root_80 : slint :: private_unstable_api :: re_exports :: r#Empty , r#rectangle_81 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#text_85 : slint :: private_unstable_api :: re_exports :: r#Text , r#touch_86 : slint :: private_unstable_api :: re_exports :: r#TouchArea , r#fs_87 : slint :: private_unstable_api :: re_exports :: r#FocusScope , r#rectangle_88 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#root_80_checkable : slint :: private_unstable_api :: re_exports :: Property < bool > , r#root_80_checked : slint :: private_unstable_api :: re_exports :: Property < bool > , r#root_80_icon : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: Image > , r#root_80_l_82_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#root_80_l_82_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_80_l_82_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_80_clicked : slint :: private_unstable_api :: re_exports :: Callback < () , () > , repeater0 : slint :: private_unstable_api :: re_exports :: Repeater < InnerComponent_image_83 > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerButton_root_80 >> , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_80 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = slint :: private_unstable_api :: re_exports :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: ModelRc :: new ({
                         let r#tmp_root_80_icon = ({
                             * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_icon }
                        ) . apply_pin (_self) . get () ;
                         ;
                         (((((r#tmp_root_80_icon . clone () . size ()) . r#width as f64) > (0f64 as f64))) && ((((r#tmp_root_80_icon . clone () . size ()) . r#height as f64) > (0f64 as f64)))) }
                     as bool)) as _ }
                 }
            ) ;
             Property :: link_two_way (({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
             + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerButton_root_80 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_83 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ) ;
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& r#repeated_indices) ;
                         r#solve_box_layout (& r#BoxLayoutData {
                             r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = r#Padding :: default () ;
                                 the_struct . r#begin = 16f64 as _ ;
                                 the_struct . r#end = 16f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerButton_root_80 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_83 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ) ;
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info (r#cells . clone () as _ , 8f64 as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 16f64 as _ ;
                             the_struct }
                         as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (1usize + _self . repeater0 . len ()) ;
                         InnerButton_root_80 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_83 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Vertical)) }
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ) ;
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 3f64 as _ ;
                             the_struct . r#end = 3f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (if ((({
                             * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
                         + r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ()) || (({
                             * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_checked }
                        ) . apply_pin (_self) . get ())) {
                             if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4293782505f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4282335039f64 as u32)) as _ }
                             }
                         else {
                             (if ({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
                             + r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                                 if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                     slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                                 else {
                                     (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                                 }
                             else {
                                 (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                     slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32) }
                                 else {
                                     (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280229663f64 as u32)) as _ }
                                ) as _ }
                            ) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4287268998f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (1f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280295198f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294243572f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (600f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                 + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (3f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
                 + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_checkable }
                            ) . apply_pin (_self) . get () {
                                 {
                                     ({
                                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_checked }
                                    ) . apply_pin (_self) . set (! ({
                                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_checked }
                                    ) . apply_pin (_self) . get () as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             ({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_clicked }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
                 + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                 + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                 + r#FocusScope :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                 + r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from (" ")))) || ((((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from ("\n"))))) {
                                 {
                                     ({
                                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
                                     + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& () . into ()) ;
                                     return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Accept) as _ ;
                                     }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Reject) as _ ;
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
             + r#FocusScope :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
             + r#FocusScope :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4278190080f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294506744f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (if ((({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) && (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
                     + r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ())) {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (6f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (6f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (3f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (3f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
             + r#FocusScope :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
             + r#FocusScope :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
             + r#FocusScope :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerButton_root_80 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_83 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_h }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_v }
                    ) . apply_pin (_self) . get ())) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = {
                             let r#minmax_lhs1 = 32f64 ;
                             ;
                             let r#minmax_rhs1 = (({
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min ;
                             ;
                             if ((r#minmax_lhs1 . clone () as f64) > (r#minmax_rhs1 . clone () as f64)) {
                                 r#minmax_lhs1 . clone () }
                             else {
                                 (r#minmax_rhs1 . clone ()) as _ }
                             }
                         as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerButton_root_80 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_83 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     slint :: private_unstable_api :: re_exports :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerButton_root_80 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_83 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater0 . component_at (subtree_index) . unwrap ())) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Button , 3usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (3usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => Default :: default () , }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_83 {
         r#image_83 : slint :: private_unstable_api :: re_exports :: r#ImageItem , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_image_83 >> , parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerButton_root_80 > , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_83 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             Property :: link_two_way (({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , (InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_icon) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ())) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#ImageFit :: r#Contain) as slint :: private_unstable_api :: re_exports :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
                 + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (3f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
             + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => {
                     let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 24f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 24f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => {
                     let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     impl InnerComponent_image_83 {
         pub fn new (parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerButton_root_80 >) -> vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerButton_root_80 > ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             slint :: private_unstable_api :: re_exports :: register_component (_self , Self :: item_array () , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
             Self :: init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [slint :: private_unstable_api :: re_exports :: ItemTreeNode] {
             const ITEM_TREE : [slint :: private_unstable_api :: re_exports :: ItemTreeNode ;
             1usize] = [slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : slint :: private_unstable_api :: re_exports :: OnceBox < [vtable :: VOffset < InnerComponent_image_83 , ItemVTable , vtable :: AllowPin > ;
             1usize] > = slint :: private_unstable_api :: re_exports :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_image_83 :: FIELD_OFFSETS . r#image_83 }
            )])) }
         }
     impl slint :: private_unstable_api :: re_exports :: PinnedDrop for InnerComponent_image_83 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_image_83 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_image_83) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             slint :: private_unstable_api :: re_exports :: unregister_component (self . as_ref () , vref , Self :: item_array () , self . window_adapter . get () . unwrap ()) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: Component for InnerComponent_image_83 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             return slint :: private_unstable_api :: re_exports :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_83 > , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> slint :: private_unstable_api :: re_exports :: Slice < slint :: private_unstable_api :: re_exports :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut slint :: private_unstable_api :: re_exports :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = slint :: private_unstable_api :: re_exports :: ItemRc :: new (parent_component , parent_index as usize + 2usize - 1) . downgrade () ;
                 }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty , result : & mut slint :: private_unstable_api :: re_exports :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: RepeatedComponent for InnerComponent_image_83 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_83 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerScrollBar_root_89 {
         r#root_89 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#handle_90 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#touch_area_91 : slint :: private_unstable_api :: re_exports :: r#TouchArea , r#root_89_enabled : slint :: private_unstable_api :: re_exports :: Property < bool > , r#root_89_horizontal : slint :: private_unstable_api :: re_exports :: Property < bool > , r#root_89_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_89_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_89_maximum : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_page_size : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_horizontal_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , r#root_89_touch_area_91_max_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_max_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_min_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_min_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_preferred_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_preferred_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_pressed_value : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_89_touch_area_91_vertical_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , r#root_89_value : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerScrollBar_root_89 >> , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerScrollBar_root_89 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_enabled }
                    ) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280229663f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (1f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                     + r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4278221012f64 as u32) }
                     else {
                         (if ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                         + r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281043160f64 as u32) }
                         else {
                             (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                            ) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((if ({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () }
                     else {
                         (({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as _ }
                     as f64) / (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (if ({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () }
                     else {
                         ({
                             let r#tmp_root_89_maximum = ({
                                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_root_89_maximum . clone () as f64) <= (((0f64 as f64) / (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_root_89_page_size = ({
                                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_page_size }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (({
                                         let r#minmax_lhs2 = 32f64 ;
                                         ;
                                         let r#minmax_rhs2 = ((({
                                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                                         + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) * (((r#tmp_root_89_page_size . clone () as f64) / (((r#tmp_root_89_maximum . clone () as f64) + (r#tmp_root_89_page_size . clone () as f64)) as f64)) as f64)) ;
                                         ;
                                         if ((r#minmax_lhs2 . clone () as f64) > (r#minmax_rhs2 . clone () as f64)) {
                                             r#minmax_lhs2 . clone () }
                                         else {
                                             (r#minmax_rhs2 . clone ()) as _ }
                                         }
                                     as f64) * (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (if ! ({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
                    ) . apply_pin (_self) . get () {
                         ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () }
                     else {
                         ({
                             let r#tmp_root_89_maximum = ({
                                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             ((if ((r#tmp_root_89_maximum . clone () as f64) <= (((0f64 as f64) / (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) as f64)) {
                                 0f64 }
                             else {
                                 ({
                                     let r#tmp_root_89_page_size = ({
                                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_page_size }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     (({
                                         let r#minmax_lhs3 = 32f64 ;
                                         ;
                                         let r#minmax_rhs3 = ((((({
                                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                                         + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) * (r#tmp_root_89_page_size . clone () as f64)) as f64) / (((r#tmp_root_89_maximum . clone () as f64) + (r#tmp_root_89_page_size . clone () as f64)) as f64)) ;
                                         ;
                                         if ((r#minmax_lhs3 . clone () as f64) > (r#minmax_rhs3 . clone () as f64)) {
                                             r#minmax_lhs3 . clone () }
                                         else {
                                             (r#minmax_rhs3 . clone ()) as _ }
                                         }
                                     as f64) * (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) }
                                ) as _ }
                             as f64) / (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) }
                        ) as _ }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((if ! ({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
                    ) . apply_pin (_self) . get () {
                         0f64 }
                     else {
                         (((((((({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_value }
                        ) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64) * (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64))) as _ }
                     as f64) / (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((if ({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
                    ) . apply_pin (_self) . get () {
                         0f64 }
                     else {
                         (((((((({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                         + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) * (((- ({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_value }
                        ) . apply_pin (_self) . get () . get () as f64) / (({
                             * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                        ) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64) * (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64))) as _ }
                     as f64) / (slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . scale_factor () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                 + r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true) && (({
                                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                             + r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 {
                                     ({
                                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_value }
                                    ) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (- {
                                         let r#minmax_lhs5 = 0f64 ;
                                         ;
                                         let r#minmax_rhs5 = {
                                             let r#minmax_lhs4 = ({
                                                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                                            ) . apply_pin (_self) . get () . get () ;
                                             ;
                                             let r#minmax_rhs4 = ((({
                                                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_pressed_value }
                                            ) . apply_pin (_self) . get () . get () as f64) + (if ({
                                                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
                                            ) . apply_pin (_self) . get () {
                                                 ((((({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                                                 + r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as f64) - (({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                                                 + r#TouchArea :: FIELD_OFFSETS . r#pressed_x) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                                                ) . apply_pin (_self) . get () . get () as f64) / (((({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                                                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                                                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64)) }
                                             else {
                                                 (((((({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                                                 + r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as f64) - (({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                                                 + r#TouchArea :: FIELD_OFFSETS . r#pressed_y) . apply_pin (_self) . get () . get () as f64)) as f64) * (((({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                                                ) . apply_pin (_self) . get () . get () as f64) / (((({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                                                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (({
                                                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
                                                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64)) as f64))) as _ }
                                             as f64)) ;
                                             ;
                                             if ((r#minmax_lhs4 . clone () as f64) < (r#minmax_rhs4 . clone () as f64)) {
                                                 r#minmax_lhs4 . clone () }
                                             else {
                                                 (r#minmax_rhs4 . clone ()) as _ }
                                             }
                                         ;
                                         ;
                                         if ((r#minmax_lhs5 . clone () as f64) > (r#minmax_rhs5 . clone () as f64)) {
                                             r#minmax_lhs5 . clone () }
                                         else {
                                             (r#minmax_rhs5 . clone ()) as _ }
                                         }
                                     as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                 + r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button) == (slint :: private_unstable_api :: re_exports :: r#PointerEventButton :: r#Left))) && ((((args . 0 . clone ()) . r#kind) == (slint :: private_unstable_api :: re_exports :: r#PointerEventKind :: r#Down)))) {
                                 {
                                     ({
                                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_touch_area_91_pressed_value }
                                    ) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (- ({
                                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_value }
                                    ) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                 + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                 + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
                 + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                     + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => ({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_layoutinfo_h }
                ) . apply_pin (_self) . get () , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => ({
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin] struct InnerAboutSlint_root_92 {
         r#root_92 : slint :: private_unstable_api :: re_exports :: r#Empty , r#t_94 : slint :: private_unstable_api :: re_exports :: r#Text , r#image_95 : slint :: private_unstable_api :: re_exports :: r#ImageItem , r#text_96 : slint :: private_unstable_api :: re_exports :: r#Text , r#root_92_empty_93_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#root_92_empty_93_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_92_empty_93_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerAboutSlint_root_92 >> , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerAboutSlint_root_92 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 48f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
                         + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = 256f64 as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 48f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (700f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("Made with")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_93_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_93_padding . clone () as f64)) as f64) - (r#tmp_empty_93_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (12f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
                 + r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#ImageFit :: r#Contain) as slint :: private_unstable_api :: re_exports :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
                 + r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if InnerPalette_99 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , Slice :: from_slice (b"svg")) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , Slice :: from_slice (b"svg"))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
                 + r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_93_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_93_padding . clone () as f64)) as f64) - (r#tmp_empty_93_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
             + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (12f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
                 + r#ImageItem :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (10f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("Version 1.0.0\nhttps://slint-ui.com/")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_93_padding = 12f64 ;
                         ;
                         ((((({
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_93_padding . clone () as f64)) as f64) - (r#tmp_empty_93_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (12f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
             + r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
             + r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
             + r#ImageItem :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => (({
                     let mut the_struct = r#LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => (({
                     let mut the_struct = r#LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layoutinfo_v }
                ) . apply_pin (_self) . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 3usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1usize , AccessibleStringProperty :: r#Label) => slint :: private_unstable_api :: re_exports :: SharedString :: from ("Made with") , (3usize , AccessibleStringProperty :: r#Label) => slint :: private_unstable_api :: re_exports :: SharedString :: from ("Version 1.0.0\nhttps://slint-ui.com/") , _ => Default :: default () , }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerApp {
         r#root_1 : slint :: private_unstable_api :: re_exports :: r#WindowItem , r#side_bar_3 : slint :: private_unstable_api :: re_exports :: r#Empty , r#rectangle_4 : slint :: private_unstable_api :: re_exports :: r#Rectangle , r#fs_5 : slint :: private_unstable_api :: re_exports :: r#FocusScope , r#label_7 : slint :: private_unstable_api :: re_exports :: r#Text , r#navigation_8 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_11 : slint :: private_unstable_api :: re_exports :: r#Empty , r#bottom_12 : slint :: private_unstable_api :: re_exports :: r#Empty , r#root_1_bottom_12_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_bottom_12_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_empty_11_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#root_1_empty_11_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_empty_11_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_empty_2_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#root_1_empty_2_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#root_1_empty_2_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_empty_2_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_empty_6_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#root_1_empty_6_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_empty_6_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_fs_5_focused_tab : slint :: private_unstable_api :: re_exports :: Property < i32 > , r#root_1_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_navigation_8_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#root_1_navigation_8_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_navigation_8_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#root_1_side_bar_3_accessible_delegate_focus : slint :: private_unstable_api :: re_exports :: Property < i32 > , r#root_1_side_bar_3_current_focused : slint :: private_unstable_api :: re_exports :: Property < i32 > , r#root_1_side_bar_3_current_item : slint :: private_unstable_api :: re_exports :: Property < i32 > , r#root_1_side_bar_3_model : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: ModelRc < slint :: private_unstable_api :: re_exports :: SharedString > > , repeater0 : slint :: private_unstable_api :: re_exports :: Repeater < InnerComponent_sidebaritem_9 > , repeater1 : slint :: private_unstable_api :: re_exports :: Repeater < InnerComponent_empty_13 > , repeater2 : slint :: private_unstable_api :: re_exports :: Repeater < InnerComponent_empty_68 > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_App }
     impl InnerApp {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = slint :: private_unstable_api :: re_exports :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: ModelRc :: new (slint :: private_unstable_api :: re_exports :: VecModel :: < slint :: private_unstable_api :: re_exports :: SharedString > :: from (slint :: private_unstable_api :: re_exports :: vec ! [slint :: private_unstable_api :: re_exports :: SharedString :: from ("Generate") as _ , slint :: private_unstable_api :: re_exports :: SharedString :: from ("About") as _]))) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = slint :: private_unstable_api :: re_exports :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: ModelRc :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item }
                    ) . apply_pin (_self) . get () as f64) == (0f64 as f64)) as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = slint :: private_unstable_api :: re_exports :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: ModelRc :: new (((({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item }
                    ) . apply_pin (_self) . get () as f64) == (1f64 as f64)) as bool)) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                 + r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#white . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_bottom_12_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& []) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_bottom_12_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& []) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_1_bottom_12_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                        ) . apply_pin (_self) . get () [5usize] as _ , r#spacing : 0f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_bottom_12_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_bottom_12_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (1usize + _self . repeater1 . len () + _self . repeater2 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 180f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 180f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerApp :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . components_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         InnerApp :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . components_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& r#repeated_indices) ;
                         r#solve_box_layout (& r#BoxLayoutData {
                             r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = r#Padding :: default () ;
                                 the_struct . r#begin = 0f64 as _ ;
                                 the_struct . r#end = 0f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                             + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (1usize + _self . repeater1 . len () + _self . repeater2 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 180f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 180f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerApp :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         InnerApp :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (1usize + _self . repeater1 . len () + _self . repeater2 . len ()) ;
                         items_vec . push ({
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ) ;
                         InnerApp :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Vertical)) }
                         InnerApp :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater2 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Vertical)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_height }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerApp :: FIELD_OFFSETS . r#label_7 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerApp :: FIELD_OFFSETS . r#label_7 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1_fs_5_focused_tab }
            ) . apply_pin (_self) . set ({
                 (0f64) as i32 }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , Slice :: from_slice (b"png"))) as slint :: private_unstable_api :: re_exports :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) + (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) + (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerApp :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_sidebaritem_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Vertical)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& r#repeated_indices) ;
                         r#solve_box_layout (& r#BoxLayoutData {
                             r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = r#Padding :: default () ;
                                 the_struct . r#begin = 0f64 as _ ;
                                 the_struct . r#end = 0f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                            ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerApp :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_sidebaritem_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerApp :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_sidebaritem_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Vertical)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_accessible_delegate_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_side_bar_3_current_focused = ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_focused }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_side_bar_3_current_focused . clone () as f64) >= (0f64 as f64)) {
                             r#tmp_side_bar_3_current_focused . clone () }
                         else {
                             (({
                                 * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item }
                            ) . apply_pin (_self) . get ()) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_focused }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
                     + r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                         ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1_fs_5_focused_tab }
                        ) . apply_pin (_self) . get () }
                     else {
                         (- 1f64) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item }
            ) . apply_pin (_self) . set ({
                 (0f64) as i32 }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_model }
            ) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: ModelRc :: new (slint :: private_unstable_api :: re_exports :: VecModel :: < slint :: private_unstable_api :: re_exports :: SharedString > :: from (slint :: private_unstable_api :: re_exports :: vec ! [slint :: private_unstable_api :: re_exports :: SharedString :: from ("Generate") as _ , slint :: private_unstable_api :: re_exports :: SharedString :: from ("About") as _]))) as slint :: private_unstable_api :: re_exports :: ModelRc < slint :: private_unstable_api :: re_exports :: SharedString > }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("Prompt Pro")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#side_bar_3 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#side_bar_3 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (180f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#side_bar_3 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#rectangle_4 }
                 + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#white . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ()) . darker (0.2f64 as f32)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#rectangle_4 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_4 }
             + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (180f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
             + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
                 + r#FocusScope :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
                 + r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from ("\n"))) {
                                 {
                                     ({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item }
                                    ) . apply_pin (_self) . set (({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_focused }
                                    ) . apply_pin (_self) . get () as _) ;
                                     return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Accept) as _ ;
                                     }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             if (((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from ("\u{f700}"))) {
                                 {
                                     ({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_1_fs_5_focused_tab }
                                    ) . apply_pin (_self) . set ({
                                         let r#minmax_lhs38 = ((({
                                             * & InnerApp :: FIELD_OFFSETS . r#root_1_fs_5_focused_tab }
                                        ) . apply_pin (_self) . get () as f64) - (1f64 as f64)) ;
                                         ;
                                         let r#minmax_rhs38 = 0f64 ;
                                         ;
                                         if ((r#minmax_lhs38 . clone () as f64) > (r#minmax_rhs38 . clone () as f64)) {
                                             r#minmax_lhs38 . clone () }
                                         else {
                                             (r#minmax_rhs38 . clone ()) as _ }
                                         }
                                     as _) ;
                                     return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Accept) as _ ;
                                     }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             if (((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from ("\u{f701}"))) {
                                 {
                                     ({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_1_fs_5_focused_tab }
                                    ) . apply_pin (_self) . set ({
                                         let r#minmax_lhs39 = ((({
                                             * & InnerApp :: FIELD_OFFSETS . r#root_1_fs_5_focused_tab }
                                        ) . apply_pin (_self) . get () as f64) + (1f64 as f64)) ;
                                         ;
                                         let r#minmax_rhs39 = ((match & ({
                                             * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_model }
                                        ) . apply_pin (_self) . get () {
                                             x => {
                                                 x . model_tracker () . track_row_count_changes () ;
                                                 x . row_count () as i32 }
                                             }
                                         as f64) - (1f64 as f64)) ;
                                         ;
                                         if ((r#minmax_lhs39 . clone () as f64) < (r#minmax_rhs39 . clone () as f64)) {
                                             r#minmax_lhs39 . clone () }
                                         else {
                                             (r#minmax_rhs39 . clone ()) as _ }
                                         }
                                     as _) ;
                                     return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Accept) as _ ;
                                     }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Reject) as _ ;
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
                 + r#FocusScope :: FIELD_OFFSETS . r#key_released) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from (" "))) {
                                 {
                                     ({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item }
                                    ) . apply_pin (_self) . set (({
                                         * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_focused }
                                    ) . apply_pin (_self) . get () as _) ;
                                     return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Accept) as _ ;
                                     }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Reject) as _ ;
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
             + r#FocusScope :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
             + r#FocusScope :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#label_7 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (16f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#label_7 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("Prompt Pro")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (180f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#label_7 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#navigation_8 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#navigation_8 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (180f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#navigation_8 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#empty_11 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#empty_11 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (180f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#empty_11 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_6_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#bottom_12 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#bottom_12 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (180f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerApp :: FIELD_OFFSETS . r#bottom_12 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_empty_11_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_model }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#side_bar_3 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#side_bar_3 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_4 }
             + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_4 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_4 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
             + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
             + r#FocusScope :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
             + r#FocusScope :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
             + r#FocusScope :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#navigation_8 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#navigation_8 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#empty_11 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#empty_11 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#bottom_12 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerApp :: FIELD_OFFSETS . r#bottom_12 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerApp :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_sidebaritem_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1usize => {
                     InnerApp :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2usize => {
                     InnerApp :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = 700f64 as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = 500f64 as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerApp :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_sidebaritem_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     slint :: private_unstable_api :: re_exports :: IndexRange :: from (_self . repeater0 . range ()) }
                 1usize => {
                     InnerApp :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     slint :: private_unstable_api :: re_exports :: IndexRange :: from (_self . repeater1 . range ()) }
                 2usize => {
                     InnerApp :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     slint :: private_unstable_api :: re_exports :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerApp :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_sidebaritem_9 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater0 . component_at (subtree_index) . unwrap ())) }
                 1usize => {
                     InnerApp :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater1 . component_at (subtree_index) . unwrap ())) }
                 2usize => {
                     InnerApp :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_68 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater2 . component_at (subtree_index) . unwrap ())) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Tab , 5usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1usize , AccessibleStringProperty :: r#DelegateFocus) => slint :: private_unstable_api :: re_exports :: SharedString :: from (slint :: private_unstable_api :: re_exports :: format ! ("{}" , ({
                     * & InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_accessible_delegate_focus }
                ) . apply_pin (_self) . get ()) . as_str ()) , (5usize , AccessibleStringProperty :: r#Label) => slint :: private_unstable_api :: re_exports :: SharedString :: from ("Prompt Pro") , _ => Default :: default () , }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_sidebaritem_9 {
         r#sidebaritem_9 : InnerSideBarItem_root_74 , r#model_data : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedString > , r#model_index : slint :: private_unstable_api :: re_exports :: Property < i32 > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_sidebaritem_9 >> , parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_sidebaritem_9 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerSideBarItem_root_74 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#sidebaritem_9 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                 + {
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
                 + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             (InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . set (({
                                 * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_index }
                            ) . apply_pin (_self) . get () as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                 + {
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get () as f64) == ((InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_focused) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                 + {
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [1usize] as usize) + ({
                             * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                 + {
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_selected }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get () as f64) == ((InnerApp :: FIELD_OFFSETS . r#root_1_side_bar_3_current_item) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () as f64))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                 + {
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (180f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                 + {
                     * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_navigation_8_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerSideBarItem_root_74 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#sidebaritem_9 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                     + {
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                     + {
                         * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (({
                             InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
                         + {
                             * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74_l_77_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#preferred as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0usize => {
                     * & Self :: FIELD_OFFSETS . r#sidebaritem_9 }
                 . apply_pin (_self) . accessible_role (0) , 1usize ..= 5usize => {
                     * & Self :: FIELD_OFFSETS . r#sidebaritem_9 }
                 . apply_pin (_self) . accessible_role (index - 1usize + 1) , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#sidebaritem_9 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1usize ..= 5usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#sidebaritem_9 }
                 . apply_pin (_self) . accessible_string_property (index - 1usize + 1 , what) , _ => Default :: default () , }
             }
         }
     impl InnerComponent_sidebaritem_9 {
         pub fn new (parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) -> vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             slint :: private_unstable_api :: re_exports :: register_component (_self , Self :: item_array () , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
             Self :: init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [slint :: private_unstable_api :: re_exports :: ItemTreeNode] {
             const ITEM_TREE : [slint :: private_unstable_api :: re_exports :: ItemTreeNode ;
             6usize] = [slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 6u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 6u32 , parent_index : 2u32 , item_array_index : 5u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : slint :: private_unstable_api :: re_exports :: OnceBox < [vtable :: VOffset < InnerComponent_sidebaritem_9 , ItemVTable , vtable :: AllowPin > ;
             6usize] > = slint :: private_unstable_api :: re_exports :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#root_74 }
            ) , VOffset :: new ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_opacity_75 }
            ) , VOffset :: new ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#l_77 }
            ) , VOffset :: new ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#touch_79 }
            ) , VOffset :: new ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#state_76 }
            ) , VOffset :: new ({
                 InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#sidebaritem_9 }
             + {
                 * & InnerSideBarItem_root_74 :: FIELD_OFFSETS . r#label_78 }
            )])) }
         }
     impl slint :: private_unstable_api :: re_exports :: PinnedDrop for InnerComponent_sidebaritem_9 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_sidebaritem_9 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_sidebaritem_9) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             slint :: private_unstable_api :: re_exports :: unregister_component (self . as_ref () , vref , Self :: item_array () , self . window_adapter . get () . unwrap ()) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: Component for InnerComponent_sidebaritem_9 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             return slint :: private_unstable_api :: re_exports :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_sidebaritem_9 > , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> slint :: private_unstable_api :: re_exports :: Slice < slint :: private_unstable_api :: re_exports :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut slint :: private_unstable_api :: re_exports :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = slint :: private_unstable_api :: re_exports :: ItemRc :: new (parent_component , parent_index as usize + 9usize - 1) . downgrade () ;
                 }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty , result : & mut slint :: private_unstable_api :: re_exports :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: RepeatedComponent for InnerComponent_sidebaritem_9 {
         type Data = slint :: private_unstable_api :: re_exports :: SharedString ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_sidebaritem_9 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_sidebaritem_9 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_empty_13 {
         r#empty_13 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_14 : slint :: private_unstable_api :: re_exports :: r#Empty , r#text_15 : slint :: private_unstable_api :: re_exports :: r#Text , r#rectangle_16 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_17 : slint :: private_unstable_api :: re_exports :: r#Empty , r#label_19 : slint :: private_unstable_api :: re_exports :: r#Text , r#rectangle_20 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_22 : slint :: private_unstable_api :: re_exports :: r#Empty , r#rectangle_23 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#inner_25 : slint :: private_unstable_api :: re_exports :: r#Empty , r#inner_clip_26 : slint :: private_unstable_api :: re_exports :: r#Clip , r#placeholder_27 : slint :: private_unstable_api :: re_exports :: r#Text , r#input_28 : slint :: private_unstable_api :: re_exports :: r#TextInput , r#empty_29 : slint :: private_unstable_api :: re_exports :: r#Empty , r#label_31 : slint :: private_unstable_api :: re_exports :: r#Text , r#rectangle_32 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_34 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_36 : slint :: private_unstable_api :: re_exports :: r#Empty , r#fs_37 : slint :: private_unstable_api :: re_exports :: r#FocusScope , r#touch_38 : slint :: private_unstable_api :: re_exports :: r#TouchArea , r#rectangle_39 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#t_41 : slint :: private_unstable_api :: re_exports :: r#Text , r#rectangle_42 : slint :: private_unstable_api :: re_exports :: r#Empty , r#path_43 : slint :: private_unstable_api :: re_exports :: r#Path , r#empty_44 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_45 : slint :: private_unstable_api :: re_exports :: r#Empty , r#label_47 : slint :: private_unstable_api :: re_exports :: r#Text , r#rectangle_48 : slint :: private_unstable_api :: re_exports :: r#Empty , r#te1_50 : slint :: private_unstable_api :: re_exports :: r#Empty , r#rectangle_51 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#fli_52 : slint :: private_unstable_api :: re_exports :: r#Flickable , r#rectangle_54 : slint :: private_unstable_api :: re_exports :: r#Rectangle , r#input_55 : slint :: private_unstable_api :: re_exports :: r#TextInput , r#corner_58 : slint :: private_unstable_api :: re_exports :: r#Rectangle , r#button_35 : InnerButton_root_80 , r#vbar_56 : InnerScrollBar_root_89 , r#hbar_57 : InnerScrollBar_root_89 , r#empty_13_empty_14_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_14_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_14_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_18_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_18_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_18_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_21_layout_cache_h : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_21_layout_cache_v : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_21_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_21_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_30_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_30_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_30_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_33_layout_cache_h : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_33_layout_cache_v : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_33_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_33_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_34_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_34_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_34_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_36_current_index : slint :: private_unstable_api :: re_exports :: Property < i32 > , r#empty_13_empty_36_model : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: ModelRc < slint :: private_unstable_api :: re_exports :: SharedString > > , r#empty_13_empty_44_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_44_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_44_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_46_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_46_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_46_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_49_layout_cache_h : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_49_layout_cache_v : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_empty_49_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_empty_49_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_inner_25_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_inner_25_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_input_28_computed_x : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_input_28_preferred_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_input_28_preferred_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_input_55_preferred_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_input_55_preferred_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_l_24_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_l_24_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_l_24_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_l_40_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_l_40_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_l_40_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_13_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_13_placeholder_27_horizontal_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , r#empty_13_placeholder_27_max_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_placeholder_27_max_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_placeholder_27_min_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_placeholder_27_min_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_placeholder_27_preferred_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_placeholder_27_preferred_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#empty_13_placeholder_27_vertical_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , r#empty_13_empty_22_accepted : slint :: private_unstable_api :: re_exports :: Callback < (slint :: private_unstable_api :: re_exports :: SharedString ,) , () > , r#empty_13_empty_22_edited : slint :: private_unstable_api :: re_exports :: Callback < (slint :: private_unstable_api :: re_exports :: SharedString ,) , () > , r#empty_13_empty_36_selected : slint :: private_unstable_api :: re_exports :: Callback < (slint :: private_unstable_api :: re_exports :: SharedString ,) , () > , r#empty_13_te1_50_edited : slint :: private_unstable_api :: re_exports :: Callback < (slint :: private_unstable_api :: re_exports :: SharedString ,) , () > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_empty_13 >> , parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_empty_13 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_80 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_35 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 18u32 - 1 , tree_index_of_first_child + 20u32 - 1) ;
             InnerScrollBar_root_89 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#vbar_56 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 38u32 - 1 , tree_index_of_first_child + 44u32 - 1) ;
             InnerScrollBar_root_89 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#hbar_57 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 39u32 - 1 , tree_index_of_first_child + 46u32 - 1) ;
             Property :: link_two_way (({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , InnerGeneratePageAdapter :: FIELD_OFFSETS . r#key_words . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_GeneratePageAdapter . as_ref ())) ;
             Property :: link_two_way (({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
             + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             Property :: link_two_way (({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_value }
            ) . apply_pin (_self) , ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self)) ;
             Property :: link_two_way (({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_value }
            ) . apply_pin (_self) , ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_13_padding = 8f64 ;
                             ;
                             (((({
                                 let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                                 * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                             as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                         as _ , r#spacing : 0f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : 50f64 as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_grid_layout (& r#GridLayoutData {
                         r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_20 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_grid_layout (& r#GridLayoutData {
                         r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = {
                                         let r#minmax_lhs34 = 32f64 ;
                                         ;
                                         let r#minmax_rhs34 = (({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_v }
                                        ) . apply_pin (_self) . get ()) . r#min ;
                                         ;
                                         if ((r#minmax_lhs34 . clone () as f64) > (r#minmax_rhs34 . clone () as f64)) {
                                             r#minmax_lhs34 . clone () }
                                         else {
                                             (r#minmax_rhs34 . clone ()) as _ }
                                         }
                                     as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 0f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#grid_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#grid_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = {
                                     let r#minmax_lhs34 = 32f64 ;
                                     ;
                                     let r#minmax_rhs34 = (({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min ;
                                     ;
                                     if ((r#minmax_lhs34 . clone () as f64) > (r#minmax_rhs34 . clone () as f64)) {
                                         r#minmax_lhs34 . clone () }
                                     else {
                                         (r#minmax_rhs34 . clone ()) as _ }
                                     }
                                 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : 50f64 as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_grid_layout (& r#GridLayoutData {
                         r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_32 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_grid_layout (& r#GridLayoutData {
                         r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 0f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#grid_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#grid_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                                 + {
                                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 170f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_h }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                             + {
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 170f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                             + {
                                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = {
                                     let r#minmax_lhs1 = 32f64 ;
                                     ;
                                     let r#minmax_rhs1 = (({
                                         InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                                     + {
                                         * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_l_82_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min ;
                                     ;
                                     if ((r#minmax_lhs1 . clone () as f64) > (r#minmax_rhs1 . clone () as f64)) {
                                         r#minmax_lhs1 . clone () }
                                     else {
                                         (r#minmax_rhs1 . clone ()) as _ }
                                     }
                                 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = {
                                     let r#minmax_lhs35 = 32f64 ;
                                     ;
                                     let r#minmax_rhs35 = (({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min ;
                                     ;
                                     if ((r#minmax_lhs35 . clone () as f64) > (r#minmax_rhs35 . clone () as f64)) {
                                         r#minmax_lhs35 . clone () }
                                     else {
                                         (r#minmax_rhs35 . clone ()) as _ }
                                     }
                                 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
            ) . apply_pin (_self) . set ({
                 (0f64) as i32 }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_model }
            ) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: ModelRc :: new (slint :: private_unstable_api :: re_exports :: VecModel :: < slint :: private_unstable_api :: re_exports :: SharedString > :: from (slint :: private_unstable_api :: re_exports :: vec ! [slint :: private_unstable_api :: re_exports :: SharedString :: from ("default") as _]))) as slint :: private_unstable_api :: re_exports :: ModelRc < slint :: private_unstable_api :: re_exports :: SharedString > }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layoutinfo_h }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_13_padding = 8f64 ;
                             ;
                             (((({
                                 let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                                 * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                             as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                         as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 16f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_44_padding = 8f64 ;
                             ;
                             ((((({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                            ) . apply_pin (_self) . get () [7usize] as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                         as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 16f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_grid_layout (& r#GridLayoutData {
                         r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 400f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_48 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_grid_layout (& r#GridLayoutData {
                         r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 0f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#grid_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 400f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#grid_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_inner_25_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_inner_25_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_55_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_55_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_inner_25_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = {
                                         let r#minmax_lhs6 = 50f64 ;
                                         ;
                                         let r#minmax_rhs6 = ({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_min_width }
                                        ) . apply_pin (_self) . get () . get () ;
                                         ;
                                         if ((r#minmax_lhs6 . clone () as f64) > (r#minmax_rhs6 . clone () as f64)) {
                                             r#minmax_lhs6 . clone () }
                                         else {
                                             (r#minmax_rhs6 . clone ()) as _ }
                                         }
                                     as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_22 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_inner_25_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = {
                                     let r#minmax_lhs6 = 50f64 ;
                                     ;
                                     let r#minmax_rhs6 = ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_min_width }
                                    ) . apply_pin (_self) . get () . get () ;
                                     ;
                                     if ((r#minmax_lhs6 . clone () as f64) > (r#minmax_rhs6 . clone () as f64)) {
                                         r#minmax_lhs6 . clone () }
                                     else {
                                         (r#minmax_rhs6 . clone ()) as _ }
                                     }
                                 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_inner_25_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_preferred_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 3f64 as _ ;
                         the_struct . r#end = 3f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
                                ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 25f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 25f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_36 }
                         + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 25f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 25f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 3f64 as _ ;
                         the_struct . r#end = 3f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 24f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 24f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 50f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 50f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layoutinfo_h }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 24f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 24f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 50f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 50f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_placeholder_27_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_14 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_14 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_14 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_14 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (20f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: SharedString :: from ("Enter a topic keyword and click the button")) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                 + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_14_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_17 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (50f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_17 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_17 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_17 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! true {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280295198f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294243572f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (600f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("key words")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_20 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_20 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_20 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_18_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_22 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_22 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_22 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_h }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_22 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280229663f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (if ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                         + r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281043160f64 as u32) }
                         else {
                             (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281479472f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                            ) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         0f64 }
                     else {
                         (if ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                         + r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (1f64) as _ }
                        ) as _ }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_25 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_25 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_25 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_25 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (3f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
                 + r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
                 + r#Clip :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
                 + r#Clip :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4284505692f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291875024f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((1f64 as f64) * (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((InnerGeneratePageAdapter :: FIELD_OFFSETS . r#key_words . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_GeneratePageAdapter . as_ref ()) . get ()) == (slint :: private_unstable_api :: re_exports :: SharedString :: from ("")))) && (((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                     + r#TextInput :: FIELD_OFFSETS . r#preedit_text) . apply_pin (_self) . get ()) == (slint :: private_unstable_api :: re_exports :: SharedString :: from (""))))) {
                         slint :: private_unstable_api :: re_exports :: SharedString :: from ("Enter some text") }
                     else {
                         (slint :: private_unstable_api :: re_exports :: SharedString :: from ("")) as _ }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#accepted) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_22_accepted }
                            ) . apply_pin (_self) . call (& (InnerGeneratePageAdapter :: FIELD_OFFSETS . r#key_words . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_GeneratePageAdapter . as_ref ()) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         InnerPalette_99 :: FIELD_OFFSETS . r#neutralPrimary . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get () }
                     else {
                         (InnerPalette_99 :: FIELD_OFFSETS . r#neutralTertiary . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ()) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_computed_x }
                            ) . apply_pin (_self) . get () . get () as f64)) as f64) < (8f64 as f64)) {
                                 {
                                     ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_computed_x }
                                    ) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((- (args . 0 . clone ()) . r#x as f64) + (8f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_computed_x }
                                ) . apply_pin (_self) . get () . get () as f64)) as f64) > (((({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                                ) . apply_pin (_self) . get () [1usize] as f64) - (8f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_computed_x }
                                        ) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                                        ) . apply_pin (_self) . get () [1usize] as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (8f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_22_edited }
                            ) . apply_pin (_self) . call (& (InnerGeneratePageAdapter :: FIELD_OFFSETS . r#key_words . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_GeneratePageAdapter . as_ref ()) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((1f64 as f64) * (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_21_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4286611584f64 as u32)) as slint :: private_unstable_api :: re_exports :: Color }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4278190080f64 as u32)) as slint :: private_unstable_api :: re_exports :: Color }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#minmax_lhs7 = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] ;
                         ;
                         let r#minmax_rhs7 = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_preferred_width }
                        ) . apply_pin (_self) . get () . get () ;
                         ;
                         if ((r#minmax_lhs7 . clone () as f64) > (r#minmax_rhs7 . clone () as f64)) {
                             r#minmax_lhs7 . clone () }
                         else {
                             (r#minmax_rhs7 . clone ()) as _ }
                         }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                 + r#TextInput :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#minmax_lhs9 = 0f64 ;
                         ;
                         let r#minmax_rhs9 = {
                             let r#minmax_lhs8 = ((({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_24_layout_cache }
                            ) . apply_pin (_self) . get () [1usize] as f64) - (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
                             + r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) ;
                             ;
                             let r#minmax_rhs8 = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_28_computed_x }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             if ((r#minmax_lhs8 . clone () as f64) > (r#minmax_rhs8 . clone () as f64)) {
                                 r#minmax_lhs8 . clone () }
                             else {
                                 (r#minmax_rhs8 . clone ()) as _ }
                             }
                         ;
                         ;
                         if ((r#minmax_lhs9 . clone () as f64) < (r#minmax_rhs9 . clone () as f64)) {
                             r#minmax_lhs9 . clone () }
                         else {
                             (r#minmax_rhs9 . clone ()) as _ }
                         }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_29 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (50f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_29 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_29 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_29 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! true {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280295198f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294243572f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (600f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("config")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_32 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_32 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_32 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_30_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_h }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                 + {
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             InnerGeneratePageAdapter :: FIELD_OFFSETS . r#generate . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_GeneratePageAdapter . as_ref ()) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
             + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (! false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                 + {
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_icon }
            ) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , Slice :: from_slice (b"png"))) as slint :: private_unstable_api :: re_exports :: Image }
            ) ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("run")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                 + {
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
                 + {
                     * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_36 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_36 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_36 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                 + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                 + r#FocusScope :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                 + r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from ("\u{f700}"))) {
                                 {
                                     ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
                                    ) . apply_pin (_self) . set ({
                                         let r#minmax_lhs36 = ((({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
                                        ) . apply_pin (_self) . get () as f64) - (1f64 as f64)) ;
                                         ;
                                         let r#minmax_rhs36 = 0f64 ;
                                         ;
                                         if ((r#minmax_lhs36 . clone () as f64) > (r#minmax_rhs36 . clone () as f64)) {
                                             r#minmax_lhs36 . clone () }
                                         else {
                                             (r#minmax_rhs36 . clone ()) as _ }
                                         }
                                     as _) ;
                                     ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                                     + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (match & ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_model }
                                    ) . apply_pin (_self) . get () {
                                         x => {
                                             let index = (({
                                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
                                            ) . apply_pin (_self) . get ()) as usize ;
                                             x . row_data_tracked (index) . unwrap_or_default () }
                                         }
                                     as _) ;
                                     return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Accept) as _ ;
                                     }
                                 }
                             else {
                                 (if (((args . 0 . clone ()) . r#text) == (slint :: private_unstable_api :: re_exports :: SharedString :: from ("\u{f701}"))) {
                                     {
                                         ({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
                                        ) . apply_pin (_self) . set ({
                                             let r#minmax_lhs37 = ((({
                                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
                                            ) . apply_pin (_self) . get () as f64) + (1f64 as f64)) ;
                                             ;
                                             let r#minmax_rhs37 = ((match & ({
                                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_model }
                                            ) . apply_pin (_self) . get () {
                                                 x => {
                                                     x . model_tracker () . track_row_count_changes () ;
                                                     x . row_count () as i32 }
                                                 }
                                             as f64) - (1f64 as f64)) ;
                                             ;
                                             if ((r#minmax_lhs37 . clone () as f64) < (r#minmax_rhs37 . clone () as f64)) {
                                                 r#minmax_lhs37 . clone () }
                                             else {
                                                 (r#minmax_rhs37 . clone ()) as _ }
                                             }
                                         as _) ;
                                         ({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                                         + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set (match & ({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_model }
                                        ) . apply_pin (_self) . get () {
                                             x => {
                                                 let index = (({
                                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
                                                ) . apply_pin (_self) . get ()) as usize ;
                                                 x . row_data_tracked (index) . unwrap_or_default () }
                                             }
                                         as _) ;
                                         return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Accept) as _ ;
                                         }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             ;
                             return (slint :: private_unstable_api :: re_exports :: r#EventResult :: r#Reject) as _ ;
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                 + r#FocusScope :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
                 + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . set_focus_item (& ItemRc :: new (VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () as usize + 26usize - 1)) ;
                             slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap () . window ()) . show_popup (& VRc :: into_dyn (InnerComponent_popup_59 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) , Point :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord , ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
                             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord) , & ItemRc :: new (VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () as usize + 19usize - 1)) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
                 + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280229663f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (if ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                         + r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281043160f64 as u32) }
                         else {
                             (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281479472f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                            ) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         0f64 }
                     else {
                         (if ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                         + r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             3f64 }
                         else {
                             (1f64) as _ }
                        ) as _ }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                     + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                         }
                     else {
                         (if ((({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                         + r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) || (({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
                         + r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                             if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281479472f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                             }
                         else {
                             (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4284505692f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291875024f64 as u32)) as _ }
                            ) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment :: r#Left) as slint :: private_unstable_api :: re_exports :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (match & slint :: private_unstable_api :: re_exports :: ModelRc :: new (slint :: private_unstable_api :: re_exports :: VecModel :: < slint :: private_unstable_api :: re_exports :: SharedString > :: from (slint :: private_unstable_api :: re_exports :: vec ! [slint :: private_unstable_api :: re_exports :: SharedString :: from ("default") as _])) {
                         x => {
                             let index = (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index }
                            ) . apply_pin (_self) . get ()) as usize ;
                             x . row_data_tracked (index) . unwrap_or_default () }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment :: r#Center) as slint :: private_unstable_api :: re_exports :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                 + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (3f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
                     + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (25f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_l_40_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (3f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#elements) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: PathData :: Events (slint :: private_unstable_api :: re_exports :: SharedVector :: < _ > :: from_slice (& Slice :: from_slice (& [slint :: private_unstable_api :: re_exports :: r#PathEvent :: r#Begin , slint :: private_unstable_api :: re_exports :: r#PathEvent :: r#Line , slint :: private_unstable_api :: re_exports :: r#PathEvent :: r#Line , slint :: private_unstable_api :: re_exports :: r#PathEvent :: r#Line , slint :: private_unstable_api :: re_exports :: r#PathEvent :: r#Line , slint :: private_unstable_api :: re_exports :: r#PathEvent :: r#Line , slint :: private_unstable_api :: re_exports :: r#PathEvent :: r#EndClosed])) , slint :: private_unstable_api :: re_exports :: SharedVector :: < _ > :: from_slice (& Slice :: from_slice (& [{
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.2199999988079071f64 as _ ;
                     the_struct . r#y = 0.4000000059604645f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.2199999988079071f64 as _ ;
                     the_struct . r#y = 0.4000000059604645f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.5f64 as _ ;
                     the_struct . r#y = 0.6399999856948853f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.5f64 as _ ;
                     the_struct . r#y = 0.6399999856948853f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.7799999713897705f64 as _ ;
                     the_struct . r#y = 0.4000000059604645f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.7799999713897705f64 as _ ;
                     the_struct . r#y = 0.4000000059604645f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.7400000095367432f64 as _ ;
                     the_struct . r#y = 0.36000001430511475f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.7400000095367432f64 as _ ;
                     the_struct . r#y = 0.36000001430511475f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.5f64 as _ ;
                     the_struct . r#y = 0.6000000238418579f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.5f64 as _ ;
                     the_struct . r#y = 0.6000000238418579f64 as _ ;
                     the_struct }
                 , {
                     let mut the_struct = r#Point :: default () ;
                     the_struct . r#x = 0.25999999046325684f64 as _ ;
                     the_struct . r#y = 0.36000001430511475f64 as _ ;
                     the_struct }
                ])))) as _ }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
                 + r#Path :: FIELD_OFFSETS . r#fill) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                     + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (25f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
                 + r#Path :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_33_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (3f64 as f64)) as f64) - (3f64 as f64)) as f64) - (8f64 as f64)) as f64) / (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_44 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                    ) . apply_pin (_self) . get () [7usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_44 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_13_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_13_padding . clone () as f64)) as f64) - (r#tmp_empty_13_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_44 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_44 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                    ) . apply_pin (_self) . get () [6usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_45 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_44_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layout_cache }
                        ) . apply_pin (_self) . get () [7usize] as f64) - (r#tmp_empty_44_padding . clone () as f64)) as f64) - (r#tmp_empty_44_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_45 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_45 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_45 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! true {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4288782237f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4291348680f64 as u32)) as _ }
                         }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280295198f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294243572f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (600f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: SharedString :: from ("output")) as slint :: private_unstable_api :: re_exports :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_48 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_48 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_44_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_48 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_46_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_h }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_v }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (if ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                         + r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281043160f64 as u32) }
                         else {
                             (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281479472f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                            ) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (if ! ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         0f64 }
                     else {
                         (if ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                         + r#TextInput :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (1f64) as _ }
                        ) as _ }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                 + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_v }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (16f64 as f64)) as f64) - (4f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                 + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_49_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as f64) - (16f64 as f64)) as f64) - (4f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + r#Flickable :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + r#Flickable :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#minmax_lhs10 = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                         + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () ;
                         ;
                         let r#minmax_rhs10 = ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_55_preferred_height }
                        ) . apply_pin (_self) . get () . get () ;
                         ;
                         if ((r#minmax_lhs10 . clone () as f64) > (r#minmax_rhs10 . clone () as f64)) {
                             r#minmax_lhs10 . clone () }
                         else {
                             (r#minmax_rhs10 . clone ()) as _ }
                         }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (if ((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . get ()) == (slint :: private_unstable_api :: re_exports :: r#TextWrap :: r#WordWrap)) {
                         ({
                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                         + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () }
                     else {
                         ({
                             let r#minmax_lhs11 = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                             + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () ;
                             ;
                             let r#minmax_rhs11 = ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_input_55_preferred_width }
                            ) . apply_pin (_self) . get () . get () ;
                             ;
                             if ((r#minmax_lhs11 . clone () as f64) > (r#minmax_rhs11 . clone () as f64)) {
                                 r#minmax_lhs11 . clone () }
                             else {
                                 (r#minmax_rhs11 . clone ()) as _ }
                             }
                        ) as _ }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (0f64 as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_54 }
                 + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         InnerPalette_99 :: FIELD_OFFSETS . r#white . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get () }
                     else {
                         (InnerPalette_99 :: FIELD_OFFSETS . r#neutralLighter . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ()) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_54 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_54 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         InnerPalette_99 :: FIELD_OFFSETS . r#neutralPrimary . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get () }
                     else {
                         (InnerPalette_99 :: FIELD_OFFSETS . r#neutralTertiary . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ()) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#cursor_position_changed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#x as f64) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                             + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . get () . get () as f64)) as f64) < (8f64 as f64)) {
                                 {
                                     ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                                         let r#minmax_lhs13 = 0f64 ;
                                         ;
                                         let r#minmax_rhs13 = {
                                             let r#minmax_lhs12 = ((({
                                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                             + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (({
                                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                             + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) ;
                                             ;
                                             let r#minmax_rhs12 = ((- (args . 0 . clone ()) . r#x as f64) + (8f64 as f64)) ;
                                             ;
                                             if ((r#minmax_lhs12 . clone () as f64) > (r#minmax_rhs12 . clone () as f64)) {
                                                 r#minmax_lhs12 . clone () }
                                             else {
                                                 (r#minmax_rhs12 . clone ()) as _ }
                                             }
                                         ;
                                         ;
                                         if ((r#minmax_lhs13 . clone () as f64) < (r#minmax_rhs13 . clone () as f64)) {
                                             r#minmax_lhs13 . clone () }
                                         else {
                                             (r#minmax_rhs13 . clone ()) as _ }
                                         }
                                     as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#x as f64) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . get () . get () as f64)) as f64) > (((({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                 + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                         + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                                             let r#minmax_lhs15 = 0f64 ;
                                             ;
                                             let r#minmax_rhs15 = {
                                                 let r#minmax_lhs14 = ((({
                                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                                 + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (({
                                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) ;
                                                 ;
                                                 let r#minmax_rhs14 = ((((({
                                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                                 + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#x as f64)) as f64) - (8f64 as f64)) ;
                                                 ;
                                                 if ((r#minmax_lhs14 . clone () as f64) > (r#minmax_rhs14 . clone () as f64)) {
                                                     r#minmax_lhs14 . clone () }
                                                 else {
                                                     (r#minmax_rhs14 . clone ()) as _ }
                                                 }
                                             ;
                                             ;
                                             if ((r#minmax_lhs15 . clone () as f64) < (r#minmax_rhs15 . clone () as f64)) {
                                                 r#minmax_lhs15 . clone () }
                                             else {
                                                 (r#minmax_rhs15 . clone ()) as _ }
                                             }
                                         as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             ;
                             if (((((args . 0 . clone ()) . r#y as f64) + (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                             + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . get () . get () as f64)) as f64) < (8f64 as f64)) {
                                 {
                                     ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                                         let r#minmax_lhs17 = 0f64 ;
                                         ;
                                         let r#minmax_rhs17 = {
                                             let r#minmax_lhs16 = ((({
                                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                             + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (({
                                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                             + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) ;
                                             ;
                                             let r#minmax_rhs16 = ((- (args . 0 . clone ()) . r#y as f64) + (8f64 as f64)) ;
                                             ;
                                             if ((r#minmax_lhs16 . clone () as f64) > (r#minmax_rhs16 . clone () as f64)) {
                                                 r#minmax_lhs16 . clone () }
                                             else {
                                                 (r#minmax_rhs16 . clone ()) as _ }
                                             }
                                         ;
                                         ;
                                         if ((r#minmax_lhs17 . clone () as f64) < (r#minmax_rhs17 . clone () as f64)) {
                                             r#minmax_lhs17 . clone () }
                                         else {
                                             (r#minmax_rhs17 . clone ()) as _ }
                                         }
                                     as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                 }
                             else {
                                 (if (((((args . 0 . clone ()) . r#y as f64) + (({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . get () . get () as f64)) as f64) > (((((({
                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                 + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (8f64 as f64)) as f64) - (20f64 as f64)) as f64)) {
                                     {
                                         ({
                                             * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                         + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                                             let r#minmax_lhs19 = 0f64 ;
                                             ;
                                             let r#minmax_rhs19 = {
                                                 let r#minmax_lhs18 = ((({
                                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                                 + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (({
                                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                                 + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) ;
                                                 ;
                                                 let r#minmax_rhs18 = ((((((({
                                                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                                                 + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - ((args . 0 . clone ()) . r#y as f64)) as f64) - (8f64 as f64)) as f64) - (20f64 as f64)) ;
                                                 ;
                                                 if ((r#minmax_lhs18 . clone () as f64) > (r#minmax_rhs18 . clone () as f64)) {
                                                     r#minmax_lhs18 . clone () }
                                                 else {
                                                     (r#minmax_rhs18 . clone ()) as _ }
                                                 }
                                             ;
                                             ;
                                             if ((r#minmax_lhs19 . clone () as f64) < (r#minmax_rhs19 . clone () as f64)) {
                                                 r#minmax_lhs19 . clone () }
                                             else {
                                                 (r#minmax_rhs19 . clone ()) as _ }
                                             }
                                         as slint :: private_unstable_api :: re_exports :: Coord) as _) }
                                     }
                                 else {
                                     ({
                                         }
                                    ) as _ }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#edited) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_te1_50_edited }
                            ) . apply_pin (_self) . call (& (({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                             + r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () as _ ,) . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (! false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#read_only) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4286611584f64 as u32)) as slint :: private_unstable_api :: re_exports :: Color }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4278190080f64 as u32)) as slint :: private_unstable_api :: re_exports :: Color }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerGeneratePageAdapter :: FIELD_OFFSETS . r#prompts . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_GeneratePageAdapter . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                 + r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: r#TextWrap :: r#WordWrap) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_enabled }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
            ) . apply_pin (_self) . set ({
                 (false) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (16f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) + (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_enabled }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (16f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_page_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (2f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
                 + {
                     * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) + (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
                 + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
                     + r#TextInput :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (0f64 as u32) }
                     else {
                         (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
             + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (16f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
             + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (16f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
                 + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) + (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
                 + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
                     + r#Flickable :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) + (2f64 as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_model }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_14 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_14 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_17 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_17 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_20 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_25 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
             + r#Clip :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
             + r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
             + r#Clip :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
             + r#Clip :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
             + r#TextInput :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_29 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_29 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_32 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_36 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
             + r#FocusScope :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
             + r#FocusScope :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
             + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#clip) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#fill_rule) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#stroke) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#stroke_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#viewbox_height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#viewbox_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#viewbox_x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#viewbox_y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
             + r#Path :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_44 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_45 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_48 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + r#Flickable :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + r#Flickable :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_54 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_54 }
             + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#input_type) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_background_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#selection_foreground_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#single_line) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#text_cursor_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
             + r#TextInput :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89_horizontal }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
             + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
             + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerButton_root_80 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_35 }
             . apply_pin (x)) ,) ;
             InnerScrollBar_root_89 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#vbar_56 }
             . apply_pin (x)) ,) ;
             InnerScrollBar_root_89 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#hbar_57 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize ..= 0usize => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_35 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0usize , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     ;
                     {
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = 100f64 as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize ..= 0usize => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_35 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0usize) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize ..= 0usize => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_35 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0usize , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 5usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 7usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 13usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 15usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 18usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Button , 19usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Combobox , 28usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 33usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 18usize => {
                     * & Self :: FIELD_OFFSETS . r#button_35 }
                 . apply_pin (_self) . accessible_role (0) , 20usize ..= 24usize => {
                     * & Self :: FIELD_OFFSETS . r#button_35 }
                 . apply_pin (_self) . accessible_role (index - 20usize + 1) , 38usize => {
                     * & Self :: FIELD_OFFSETS . r#vbar_56 }
                 . apply_pin (_self) . accessible_role (0) , 44usize ..= 45usize => {
                     * & Self :: FIELD_OFFSETS . r#vbar_56 }
                 . apply_pin (_self) . accessible_role (index - 44usize + 1) , 39usize => {
                     * & Self :: FIELD_OFFSETS . r#hbar_57 }
                 . apply_pin (_self) . accessible_role (0) , 46usize ..= 47usize => {
                     * & Self :: FIELD_OFFSETS . r#hbar_57 }
                 . apply_pin (_self) . accessible_role (index - 46usize + 1) , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (5usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (7usize , AccessibleStringProperty :: r#Label) => slint :: private_unstable_api :: re_exports :: SharedString :: from ("key words") , (13usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (15usize , AccessibleStringProperty :: r#Label) => slint :: private_unstable_api :: re_exports :: SharedString :: from ("config") , (18usize , AccessibleStringProperty :: r#Label) => slint :: private_unstable_api :: re_exports :: SharedString :: from ("run") , (19usize , AccessibleStringProperty :: r#Value) => ({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (28usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (33usize , AccessibleStringProperty :: r#Label) => slint :: private_unstable_api :: re_exports :: SharedString :: from ("output") , (18usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_35 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (20usize ..= 24usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_35 }
                 . apply_pin (_self) . accessible_string_property (index - 20usize + 1 , what) , (38usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#vbar_56 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (44usize ..= 45usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#vbar_56 }
                 . apply_pin (_self) . accessible_string_property (index - 44usize + 1 , what) , (39usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#hbar_57 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (46usize ..= 47usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#hbar_57 }
                 . apply_pin (_self) . accessible_string_property (index - 46usize + 1 , what) , _ => Default :: default () , }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_popup_59 {
         r#popup_59 : slint :: private_unstable_api :: re_exports :: r#WindowItem , r#rectangle_60 : slint :: private_unstable_api :: re_exports :: r#BorderRectangle , r#popup_59_empty_61_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#popup_59_empty_61_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#popup_59_empty_61_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#popup_59_empty_61_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#popup_59_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#popup_59_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , repeater0 : slint :: private_unstable_api :: re_exports :: Repeater < InnerComponent_rectangle_62 > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_popup_59 >> , parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_empty_13 > , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_popup_59 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = slint :: private_unstable_api :: re_exports :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: ModelRc :: new (slint :: private_unstable_api :: re_exports :: VecModel :: < slint :: private_unstable_api :: re_exports :: SharedString > :: from (slint :: private_unstable_api :: re_exports :: vec ! [slint :: private_unstable_api :: re_exports :: SharedString :: from ("default") as _]))) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerComponent_popup_59 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Vertical)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& r#repeated_indices) ;
                         r#solve_box_layout (& r#BoxLayoutData {
                             r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = r#Padding :: default () ;
                                 the_struct . r#begin = 0f64 as _ ;
                                 the_struct . r#end = 0f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                             + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerComponent_popup_59 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = slint :: private_unstable_api :: re_exports :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerComponent_popup_59 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . components_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (slint :: private_unstable_api :: re_exports :: Orientation :: Vertical)) }
                         let r#cells = slint :: private_unstable_api :: re_exports :: Slice :: from_slice (& items_vec) ;
                         r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                 + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#minmax_lhs55 = (({
                             * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#preferred ;
                         ;
                         let r#minmax_rhs55 = (({
                             * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min ;
                         ;
                         if ((r#minmax_lhs55 . clone () as f64) > (r#minmax_rhs55 . clone () as f64)) {
                             r#minmax_lhs55 . clone () }
                         else {
                             (r#minmax_rhs55 . clone ()) as _ }
                         }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) + (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) + (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                 + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_34_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294967295f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280229663f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                         slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                     else {
                         (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (1f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
                 + r#BorderRectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
             + r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
             + r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
             + r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
             + r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
             + r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
             + r#BorderRectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerComponent_popup_59 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => ({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_layoutinfo_h }
                ) . apply_pin (_self) . get () , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => ({
                     * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerComponent_popup_59 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     slint :: private_unstable_api :: re_exports :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0usize => {
                     InnerComponent_popup_59 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_62 :: new (_self . self_weak . get () . unwrap () . clone ()) . into ()) ;
                     * result = vtable :: VRc :: downgrade (& vtable :: VRc :: into_dyn (_self . repeater0 . component_at (subtree_index) . unwrap ())) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => Default :: default () , }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_62 {
         r#rectangle_62 : slint :: private_unstable_api :: re_exports :: r#Rectangle , r#text_64 : slint :: private_unstable_api :: re_exports :: r#Text , r#item_area_65 : slint :: private_unstable_api :: re_exports :: r#TouchArea , r#model_data : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedString > , r#model_index : slint :: private_unstable_api :: re_exports :: Property < i32 > , r#rectangle_62_empty_63_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#rectangle_62_empty_63_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#rectangle_62_empty_63_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#rectangle_62_item_area_65_horizontal_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , r#rectangle_62_item_area_65_max_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#rectangle_62_item_area_65_max_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#rectangle_62_item_area_65_min_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#rectangle_62_item_area_65_min_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#rectangle_62_item_area_65_preferred_height : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#rectangle_62_item_area_65_preferred_width : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: LogicalLength > , r#rectangle_62_item_area_65_vertical_stretch : slint :: private_unstable_api :: re_exports :: Property < f32 > , r#rectangle_62_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#rectangle_62_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_rectangle_62 >> , parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_popup_59 > , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_62 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
                 + r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ((({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_index }
                    ) . apply_pin (_self) . get () as f64) == ((InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref () . parent . upgrade () . unwrap () . as_pin_ref ()) . get () as f64)) {
                         if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                             slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294177521f64 as u32) }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4281413937f64 as u32)) as _ }
                         }
                     else {
                         (if ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                         + r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             if ! InnerColorSchemeSelector_98 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_98 . as_ref ()) . get () {
                                 slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4294638072f64 as u32) }
                             else {
                                 (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (4280821800f64 as u32)) as _ }
                             }
                         else {
                             (slint :: private_unstable_api :: re_exports :: Color :: from_argb_encoded (0f64 as u32)) as _ }
                        ) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_empty_63_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
                         + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_empty_63_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_empty_63_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
                 + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [1usize] as usize) + ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#max as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#min as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#preferred as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                    ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ())) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_empty_63_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_empty_63_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + ({
                         let mut the_struct = r#LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_item_area_65_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
                 + r#Rectangle :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
                 + r#Rectangle :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                 + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_empty_63_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_63_padding = 10f64 ;
                         ;
                         (((((InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59_empty_61_width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (r#tmp_empty_63_padding . clone () as f64)) as f64) - (r#tmp_empty_63_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (10f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
                 + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_empty_63_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                 + r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ({
                                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
                             + r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref () . parent . upgrade () . unwrap () . as_pin_ref ()) . get () {
                                 {
                                     (InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_current_index) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref () . parent . upgrade () . unwrap () . as_pin_ref ()) . set (({
                                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_index }
                                    ) . apply_pin (_self) . get () as _) ;
                                     ({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                                     + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref () . parent . upgrade () . unwrap () . as_pin_ref ()) . set (({
                                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_data }
                                    ) . apply_pin (_self) . get () as _) ;
                                     (InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13_empty_36_selected) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref () . parent . upgrade () . unwrap () . as_pin_ref ()) . call (& (({
                                         * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
                                     + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref () . parent . upgrade () . unwrap () . as_pin_ref ()) . get () as _ ,) . into ()) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                 + r#TouchArea :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
                     + r#Rectangle :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
                 + r#TouchArea :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (((1f64 as f64) * (({
                         * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
                     + r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
             + r#Rectangle :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
             + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
             + r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
             + r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
             + r#TouchArea :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
             + r#TouchArea :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => ({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_layoutinfo_h }
                ) . apply_pin (_self) . get () , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => ({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_data }
                ) . apply_pin (_self) . get () , _ => Default :: default () , }
             }
         }
     impl InnerComponent_rectangle_62 {
         pub fn new (parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_popup_59 >) -> vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_popup_59 > ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             slint :: private_unstable_api :: re_exports :: register_component (_self , Self :: item_array () , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
             Self :: init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [slint :: private_unstable_api :: re_exports :: ItemTreeNode] {
             const ITEM_TREE : [slint :: private_unstable_api :: re_exports :: ItemTreeNode ;
             3usize] = [slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 2u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : slint :: private_unstable_api :: re_exports :: OnceBox < [vtable :: VOffset < InnerComponent_rectangle_62 , ItemVTable , vtable :: AllowPin > ;
             3usize] > = slint :: private_unstable_api :: re_exports :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#rectangle_62 }
            ) , VOffset :: new ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#text_64 }
            ) , VOffset :: new ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#item_area_65 }
            )])) }
         }
     impl slint :: private_unstable_api :: re_exports :: PinnedDrop for InnerComponent_rectangle_62 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_62 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_rectangle_62) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             slint :: private_unstable_api :: re_exports :: unregister_component (self . as_ref () , vref , Self :: item_array () , self . window_adapter . get () . unwrap ()) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: Component for InnerComponent_rectangle_62 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             return slint :: private_unstable_api :: re_exports :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_62 > , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> slint :: private_unstable_api :: re_exports :: Slice < slint :: private_unstable_api :: re_exports :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut slint :: private_unstable_api :: re_exports :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = slint :: private_unstable_api :: re_exports :: ItemRc :: new (parent_component , parent_index as usize + 2usize - 1) . downgrade () ;
                 }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty , result : & mut slint :: private_unstable_api :: re_exports :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: RepeatedComponent for InnerComponent_rectangle_62 {
         type Data = slint :: private_unstable_api :: re_exports :: SharedString ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_rectangle_62 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_62 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerComponent_popup_59 {
         pub fn new (parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_empty_13 >) -> vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_empty_13 > ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             slint :: private_unstable_api :: re_exports :: register_component (_self , Self :: item_array () , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
             Self :: init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             Self :: user_init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x)) ;
             self_rc }
         fn item_tree () -> & 'static [slint :: private_unstable_api :: re_exports :: ItemTreeNode] {
             const ITEM_TREE : [slint :: private_unstable_api :: re_exports :: ItemTreeNode ;
             3usize] = [slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: DynamicTree {
                 index : 0usize , parent_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : slint :: private_unstable_api :: re_exports :: OnceBox < [vtable :: VOffset < InnerComponent_popup_59 , ItemVTable , vtable :: AllowPin > ;
             2usize] > = slint :: private_unstable_api :: re_exports :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#popup_59 }
            ) , VOffset :: new ({
                 * & InnerComponent_popup_59 :: FIELD_OFFSETS . r#rectangle_60 }
            )])) }
         }
     impl slint :: private_unstable_api :: re_exports :: PinnedDrop for InnerComponent_popup_59 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_popup_59 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_popup_59) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             slint :: private_unstable_api :: re_exports :: unregister_component (self . as_ref () , vref , Self :: item_array () , self . window_adapter . get () . unwrap ()) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: Component for InnerComponent_popup_59 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             return slint :: private_unstable_api :: re_exports :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_popup_59 > , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> slint :: private_unstable_api :: re_exports :: Slice < slint :: private_unstable_api :: re_exports :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut slint :: private_unstable_api :: re_exports :: ItemWeak) {
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty , result : & mut slint :: private_unstable_api :: re_exports :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     impl InnerComponent_empty_13 {
         pub fn new (parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) -> vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             slint :: private_unstable_api :: re_exports :: register_component (_self , Self :: item_array () , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
             Self :: init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [slint :: private_unstable_api :: re_exports :: ItemTreeNode] {
             const ITEM_TREE : [slint :: private_unstable_api :: re_exports :: ItemTreeNode ;
             48usize] = [slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 7u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 15u32 , parent_index : 0u32 , item_array_index : 3u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 32u32 , parent_index : 0u32 , item_array_index : 4u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 7u32 , parent_index : 1u32 , item_array_index : 6u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 9u32 , parent_index : 2u32 , item_array_index : 7u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 9u32 , parent_index : 2u32 , item_array_index : 8u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 10u32 , parent_index : 8u32 , item_array_index : 9u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 12u32 , parent_index : 9u32 , item_array_index : 10u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 12u32 , parent_index : 9u32 , item_array_index : 11u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 13u32 , parent_index : 11u32 , item_array_index : 12u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 15u32 , parent_index : 12u32 , item_array_index : 13u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 15u32 , parent_index : 12u32 , item_array_index : 14u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 17u32 , parent_index : 3u32 , item_array_index : 15u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 3u32 , item_array_index : 16u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 18u32 , parent_index : 16u32 , item_array_index : 17u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 6u32 , children_index : 20u32 , parent_index : 17u32 , item_array_index : 18u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 26u32 , parent_index : 17u32 , item_array_index : 19u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 26u32 , parent_index : 18u32 , item_array_index : 20u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: DynamicTree {
                 index : 0usize , parent_index : 18u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 26u32 , parent_index : 18u32 , item_array_index : 21u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 26u32 , parent_index : 18u32 , item_array_index : 22u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 26u32 , parent_index : 18u32 , item_array_index : 23u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 26u32 , parent_index : 18u32 , item_array_index : 24u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 30u32 , parent_index : 19u32 , item_array_index : 25u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 19u32 , item_array_index : 26u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 31u32 , parent_index : 19u32 , item_array_index : 27u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 31u32 , parent_index : 19u32 , item_array_index : 28u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 31u32 , parent_index : 26u32 , item_array_index : 29u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 32u32 , parent_index : 29u32 , item_array_index : 30u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 33u32 , parent_index : 4u32 , item_array_index : 31u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 35u32 , parent_index : 32u32 , item_array_index : 32u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 35u32 , parent_index : 32u32 , item_array_index : 33u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 5u32 , children_index : 36u32 , parent_index : 34u32 , item_array_index : 34u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 41u32 , parent_index : 35u32 , item_array_index : 35u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 41u32 , parent_index : 35u32 , item_array_index : 36u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 44u32 , parent_index : 35u32 , item_array_index : 37u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 46u32 , parent_index : 35u32 , item_array_index : 38u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 48u32 , parent_index : 35u32 , item_array_index : 39u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 42u32 , parent_index : 37u32 , item_array_index : 40u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 44u32 , parent_index : 41u32 , item_array_index : 41u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 44u32 , parent_index : 41u32 , item_array_index : 42u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 46u32 , parent_index : 38u32 , item_array_index : 43u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 46u32 , parent_index : 38u32 , item_array_index : 44u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 48u32 , parent_index : 39u32 , item_array_index : 45u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 48u32 , parent_index : 39u32 , item_array_index : 46u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : slint :: private_unstable_api :: re_exports :: OnceBox < [vtable :: VOffset < InnerComponent_empty_13 , ItemVTable , vtable :: AllowPin > ;
             47usize] > = slint :: private_unstable_api :: re_exports :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_13 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_14 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_17 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_29 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_44 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#text_15 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_16 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_19 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_20 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_22 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_23 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_25 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#inner_clip_26 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#placeholder_27 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_28 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_31 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_32 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_34 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#root_80 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_36 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_81 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#text_85 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#touch_86 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#fs_87 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#button_35 }
             + {
                 * & InnerButton_root_80 :: FIELD_OFFSETS . r#rectangle_88 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fs_37 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_39 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#t_41 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_42 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#touch_38 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#path_43 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#empty_45 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#label_47 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_48 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#te1_50 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_51 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#root_89 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#corner_58 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#fli_52 }
             + slint :: private_unstable_api :: re_exports :: Flickable :: FIELD_OFFSETS . viewport) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#rectangle_54 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_13 :: FIELD_OFFSETS . r#input_55 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#vbar_56 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#handle_90 }
            ) , VOffset :: new ({
                 InnerComponent_empty_13 :: FIELD_OFFSETS . r#hbar_57 }
             + {
                 * & InnerScrollBar_root_89 :: FIELD_OFFSETS . r#touch_area_91 }
            )])) }
         }
     impl slint :: private_unstable_api :: re_exports :: PinnedDrop for InnerComponent_empty_13 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_empty_13 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_empty_13) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             slint :: private_unstable_api :: re_exports :: unregister_component (self . as_ref () , vref , Self :: item_array () , self . window_adapter . get () . unwrap ()) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: Component for InnerComponent_empty_13 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             return slint :: private_unstable_api :: re_exports :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_empty_13 > , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> slint :: private_unstable_api :: re_exports :: Slice < slint :: private_unstable_api :: re_exports :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut slint :: private_unstable_api :: re_exports :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = slint :: private_unstable_api :: re_exports :: ItemRc :: new (parent_component , parent_index as usize + 2usize - 1) . downgrade () ;
                 }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty , result : & mut slint :: private_unstable_api :: re_exports :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: RepeatedComponent for InnerComponent_empty_13 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_empty_13 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (slint :: private_unstable_api :: re_exports :: FieldOffsets , Default)] # [const_field_offset (slint :: private_unstable_api :: re_exports :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_empty_68 {
         r#empty_68 : slint :: private_unstable_api :: re_exports :: r#Empty , r#empty_69 : slint :: private_unstable_api :: re_exports :: r#Empty , r#text_70 : slint :: private_unstable_api :: re_exports :: r#Text , r#rectangle_71 : slint :: private_unstable_api :: re_exports :: r#Empty , r#aboutslint_72 : InnerAboutSlint_root_92 , r#empty_68_empty_69_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_68_empty_69_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_68_empty_69_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_68_layout_cache : slint :: private_unstable_api :: re_exports :: Property < slint :: private_unstable_api :: re_exports :: SharedVector < slint :: private_unstable_api :: re_exports :: Coord , > > , r#empty_68_layoutinfo_h : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , r#empty_68_layoutinfo_v : slint :: private_unstable_api :: re_exports :: Property < r#LayoutInfo > , self_weak : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeakMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerComponent_empty_68 >> , parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , window_adapter : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: Rc < dyn slint :: private_unstable_api :: re_exports :: WindowAdapter >> , root : slint :: private_unstable_api :: re_exports :: OnceCell < slint :: private_unstable_api :: re_exports :: VWeak < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_empty_68 {
         pub fn init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , root : & slint :: private_unstable_api :: re_exports :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (VRc :: downgrade (root)) ;
             _self . window_adapter . set (root . window_adapter . get () . unwrap () . clone ()) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerAboutSlint_root_92 :: init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#aboutslint_72 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 5u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
                            ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : {
                             let r#tmp_empty_68_padding = 8f64 ;
                             ;
                             (((({
                                 let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                                 * cache . get ((cache [5usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                             as f64) - (r#tmp_empty_68_padding . clone () as f64)) as f64) - (r#tmp_empty_68_padding . clone () as f64)) }
                         as _ , r#spacing : 0f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = slint :: private_unstable_api :: re_exports :: Item :: layout_info (({
                             * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
                        ) . apply_pin (_self) , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical , _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                     + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#solve_box_layout (& r#BoxLayoutData {
                         r#alignment : slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _ , r#cells : Slice :: from_slice (& [{
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = r#LayoutInfo :: default () ;
                                     the_struct . r#max = 24f64 as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 24f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = r#BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = (({
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
                             + {
                                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = r#Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerApp :: FIELD_OFFSETS . r#root_1 }
                         + r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info_ortho (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = r#LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
                         + {
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layoutinfo_h }
                        ) . apply_pin (_self) . get ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (r#box_layout_info (Slice :: from_slice (& [{
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = r#LayoutInfo :: default () ;
                                 the_struct . r#max = 24f64 as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 24f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = r#BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = r#LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
                         + {
                             * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92_empty_93_layoutinfo_v }
                        ) . apply_pin (_self) . get ())) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = r#Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , slint :: private_unstable_api :: re_exports :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [5usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [4usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_69 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_69 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_68_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [5usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_68_padding . clone () as f64)) as f64) - (r#tmp_empty_68_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_69 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_69 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                 + r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (InnerPalette_99 :: FIELD_OFFSETS . r#neutralDark . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_Palette_99 . as_ref ()) . get ())) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (20f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: SharedString :: from ("About")) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                 + r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                 + r#Text :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (24f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
                 + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_empty_69_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
                 + {
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
                 + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
                 + {
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
                 + r#Empty :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new ({
                         let r#tmp_empty_68_padding = 8f64 ;
                         ;
                         (((({
                             let cache = (InnerApp :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                             * cache . get ((cache [5usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as slint :: private_unstable_api :: re_exports :: Coord)) }
                         as f64) - (r#tmp_empty_68_padding . clone () as f64)) as f64) - (r#tmp_empty_68_padding . clone () as f64)) }
                     as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
             + {
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set ({
                 (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (8f64 as slint :: private_unstable_api :: re_exports :: Coord)) as slint :: private_unstable_api :: re_exports :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
                 + {
                     * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
                 + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: private_unstable_api :: re_exports :: LogicalLength :: new (({
                         * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as slint :: private_unstable_api :: re_exports :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_69 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_69 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
             + r#Text :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
             + r#Empty :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
             + r#Empty :: FIELD_OFFSETS . r#y) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
             + {
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
             + r#Empty :: FIELD_OFFSETS . r#x) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : slint :: private_unstable_api :: re_exports :: VRcMapped < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             InnerAboutSlint_root_92 :: user_init (VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#aboutslint_72 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 slint :: private_unstable_api :: re_exports :: Orientation :: Horizontal => ({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layoutinfo_h }
                ) . apply_pin (_self) . get () , slint :: private_unstable_api :: re_exports :: Orientation :: Vertical => ({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 3usize => slint :: private_unstable_api :: re_exports :: r#AccessibleRole :: r#Text , 2usize => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_72 }
                 . apply_pin (_self) . accessible_role (0) , 5usize ..= 7usize => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_72 }
                 . apply_pin (_self) . accessible_role (index - 5usize + 1) , _ => AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty ,) -> slint :: private_unstable_api :: re_exports :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (3usize , AccessibleStringProperty :: r#Label) => ({
                     * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
                 + r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (2usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_72 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (5usize ..= 7usize , _) => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_72 }
                 . apply_pin (_self) . accessible_string_property (index - 5usize + 1 , what) , _ => Default :: default () , }
             }
         }
     impl InnerComponent_empty_68 {
         pub fn new (parent : slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) -> vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > {
             # ! [allow (unused)] let mut _self = Self :: default () ;
             _self . parent = parent . clone () as slint :: private_unstable_api :: re_exports :: VWeakMapped :: < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             slint :: private_unstable_api :: re_exports :: register_component (_self , Self :: item_array () , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () . window_adapter . get () . unwrap ()) ;
             Self :: init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             self_rc }
         fn item_tree () -> & 'static [slint :: private_unstable_api :: re_exports :: ItemTreeNode] {
             const ITEM_TREE : [slint :: private_unstable_api :: re_exports :: ItemTreeNode ;
             8usize] = [slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 5u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 5u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 8u32 , parent_index : 2u32 , item_array_index : 5u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 8u32 , parent_index : 2u32 , item_array_index : 6u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 8u32 , parent_index : 2u32 , item_array_index : 7u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : slint :: private_unstable_api :: re_exports :: OnceBox < [vtable :: VOffset < InnerComponent_empty_68 , ItemVTable , vtable :: AllowPin > ;
             8usize] > = slint :: private_unstable_api :: re_exports :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_68 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#empty_69 }
            ) , VOffset :: new ({
                 InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
             + {
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#root_92 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#text_70 }
            ) , VOffset :: new ({
                 * & InnerComponent_empty_68 :: FIELD_OFFSETS . r#rectangle_71 }
            ) , VOffset :: new ({
                 InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
             + {
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#t_94 }
            ) , VOffset :: new ({
                 InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
             + {
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#image_95 }
            ) , VOffset :: new ({
                 InnerComponent_empty_68 :: FIELD_OFFSETS . r#aboutslint_72 }
             + {
                 * & InnerAboutSlint_root_92 :: FIELD_OFFSETS . r#text_96 }
            )])) }
         }
     impl slint :: private_unstable_api :: re_exports :: PinnedDrop for InnerComponent_empty_68 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_empty_68 >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerComponent_empty_68) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             slint :: private_unstable_api :: re_exports :: unregister_component (self . as_ref () , vref , Self :: item_array () , self . window_adapter . get () . unwrap ()) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: Component for InnerComponent_empty_68 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             return slint :: private_unstable_api :: re_exports :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_empty_68 > , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> slint :: private_unstable_api :: re_exports :: Slice < slint :: private_unstable_api :: re_exports :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut slint :: private_unstable_api :: re_exports :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = slint :: private_unstable_api :: re_exports :: ItemRc :: new (parent_component , parent_index as usize + 3usize - 1) . downgrade () ;
                 }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty , result : & mut slint :: private_unstable_api :: re_exports :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: RepeatedComponent for InnerComponent_empty_68 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_empty_68 :: user_init (VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: BoxLayoutCellData {
             BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerApp {
         pub fn new () -> core :: result :: Result < vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , Self > , slint :: PlatformError , > {
             # ! [allow (unused)] let window_adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
             let mut _self = Self :: default () ;
             let self_rc = VRc :: new (_self) ;
             let _self = self_rc . as_pin_ref () ;
             _self . window_adapter . set (window_adapter) ;
             slint :: private_unstable_api :: re_exports :: WindowInner :: from_pub (_self . window_adapter . get () . unwrap () . window ()) . set_component (& VRc :: into_dyn (self_rc . clone ())) ;
             slint :: private_unstable_api :: re_exports :: register_component (_self , Self :: item_array () , & self_rc . window_adapter . get () . unwrap ()) ;
             Self :: init (slint :: private_unstable_api :: re_exports :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [slint :: private_unstable_api :: re_exports :: ItemTreeNode] {
             const ITEM_TREE : [slint :: private_unstable_api :: re_exports :: ItemTreeNode ;
             11usize] = [slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 4u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: DynamicTree {
                 index : 1usize , parent_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: DynamicTree {
                 index : 2usize , parent_index : 0u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 8u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 9u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 9u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 10u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 9u32 , parent_index : 4u32 , item_array_index : 6u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: DynamicTree {
                 index : 0usize , parent_index : 6u32 , }
             , slint :: private_unstable_api :: re_exports :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 11u32 , parent_index : 7u32 , item_array_index : 7u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [vtable :: VOffset < Self , ItemVTable , vtable :: AllowPin >] {
             static ITEM_ARRAY : slint :: private_unstable_api :: re_exports :: OnceBox < [vtable :: VOffset < InnerApp , ItemVTable , vtable :: AllowPin > ;
             8usize] > = slint :: private_unstable_api :: re_exports :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| Box :: new ([VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#root_1 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#side_bar_3 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#rectangle_4 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#label_7 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#navigation_8 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#empty_11 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#fs_5 }
            ) , VOffset :: new ({
                 * & InnerApp :: FIELD_OFFSETS . r#bottom_12 }
            )])) }
         }
     impl slint :: private_unstable_api :: re_exports :: PinnedDrop for InnerApp {
         fn drop (self : core :: pin :: Pin < & mut InnerApp >) {
             use slint :: private_unstable_api :: re_exports :: * ;
             ComponentVTable_static ! (static VT for self :: InnerApp) ;
             new_vref ! (let vref : VRef < ComponentVTable > for Component = self . as_ref () . get_ref ()) ;
             slint :: private_unstable_api :: re_exports :: unregister_component (self . as_ref () , vref , Self :: item_array () , self . window_adapter . get () . unwrap ()) ;
             }
         }
     impl slint :: private_unstable_api :: re_exports :: Component for InnerApp {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : slint :: private_unstable_api :: re_exports :: ItemVisitorRefMut) -> slint :: private_unstable_api :: re_exports :: VisitChildrenResult {
             return slint :: private_unstable_api :: re_exports :: visit_item_tree (self , & VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerApp > , order : slint :: private_unstable_api :: re_exports :: TraversalOrder , visitor : ItemVisitorRefMut , dyn_index : usize) -> VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : usize) -> :: core :: pin :: Pin < ItemRef > {
             match & self . get_item_tree () . as_slice () [index] {
                 ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> slint :: private_unstable_api :: re_exports :: Slice < slint :: private_unstable_api :: re_exports :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree_component (self : :: core :: pin :: Pin < & Self > , index : usize , subtree_index : usize , result : & mut slint :: private_unstable_api :: re_exports :: ComponentWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut slint :: private_unstable_api :: re_exports :: ItemWeak) {
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : slint :: private_unstable_api :: re_exports :: Orientation) -> slint :: private_unstable_api :: re_exports :: LayoutInfo {
             self . layout_info (orientation) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : usize) -> slint :: private_unstable_api :: re_exports :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : usize , what : slint :: private_unstable_api :: re_exports :: AccessibleStringProperty , result : & mut slint :: private_unstable_api :: re_exports :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         }
     pub struct r#App (vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) ;
     impl r#App {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerApp :: new () ? ;
             inner . globals . global_GeneratePageAdapter . clone () . init (& inner) ;
             inner . globals . global_ColorSchemeSelector_98 . clone () . init (& inner) ;
             inner . globals . global_Palette_99 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_100 . clone () . init (& inner) ;
             InnerApp :: user_init (slint :: private_unstable_api :: re_exports :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         }
     impl From < r#App > for vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp > {
         fn from (value : r#App) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#App {
         type Inner = InnerApp ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : vtable :: VRc < slint :: private_unstable_api :: re_exports :: ComponentVTable , InnerApp >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . window () . hide () }
         fn window (& self) -> & slint :: Window {
             vtable :: VRc :: as_pin_ref (& self . 0) . get_ref () . window_adapter . get () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_App {
         global_GeneratePageAdapter : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < InnerGeneratePageAdapter >> , global_ColorSchemeSelector_98 : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < InnerColorSchemeSelector_98 >> , global_Palette_99 : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < InnerPalette_99 >> , global_StyleMetrics_100 : :: core :: pin :: Pin < slint :: private_unstable_api :: re_exports :: Rc < InnerStyleMetrics_100 >> , }
     impl Default for Globals_App {
         fn default () -> Self {
             Self {
                 global_GeneratePageAdapter : InnerGeneratePageAdapter :: new () , global_ColorSchemeSelector_98 : InnerColorSchemeSelector_98 :: new () , global_Palette_99 : InnerPalette_99 :: new () , global_StyleMetrics_100 : InnerStyleMetrics_100 :: new () , }
             }
         }
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = :: core :: include_bytes ! ("/Users/liufankai/Desktop/code/ai/Promptpro/ui/pages/../../logo.png") ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"215\" height=\"93\" viewBox=\"0 0 215 93\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M10.2898 89.0771H57.4131C61.4796 89.0771 64.7761 85.7726 64.7761 81.6962L64.7761 11.5771C64.7761 7.50076 61.4796 4.1962 57.4131 4.1962H10.2898C6.22334 4.1962 2.9268 7.50076 2.9268 11.5771L2.9268 81.6962C2.9268 85.7726 6.22334 89.0771 10.2898 89.0771Z\" fill=\"url(#paint0_radial_1014_7422)\"/>\n<path d=\"M18.113 77.1532L53.5033 52.5894C53.5033 52.5894 55.1011 51.6705 55.1011 50.2201C55.1011 48.29 53.08 47.6626 53.08 47.6626L33.6085 39.9384C32.9164 39.669 31.9592 40.433 32.8538 41.4146L39.3038 47.8951C39.3038 47.8951 41.1077 49.6555 41.1077 50.8143C41.1077 51.9731 40.0033 53.0027 40.0033 53.0027L16.5373 75.6549C15.7016 76.4778 16.8282 77.943 18.113 77.1532Z\" fill=\"#0025FF\"/>\n<path d=\"M46.6999 14.8654L11.3059 39.4329C11.3059 39.4329 9.70813 40.3518 9.70813 41.8022C9.70813 43.7323 11.7293 44.3597 11.7293 44.3597L31.2008 52.0839C31.8966 52.3533 32.8538 51.5893 31.9371 50.6077L25.5055 44.1383C25.5055 44.1383 23.7015 42.3816 23.7015 41.2228C23.7015 40.064 24.806 39.0343 24.806 39.0343L48.283 16.3527C49.1113 15.5444 47.9848 14.0793 46.6999 14.8654Z\" fill=\"#0025FF\"/>\n<path d=\"M79.1524 69.322H77.9744C77.801 69.3387 77.6281 69.2871 77.4921 69.1781C77.3916 69.0633 77.34 68.9136 77.3485 68.7611C77.3479 68.6357 77.3678 68.511 77.4074 68.392C77.4342 68.3149 77.4662 68.2397 77.5031 68.1669H81.5528C81.7262 68.1494 81.8994 68.2012 82.0351 68.3108C82.0862 68.3668 82.1254 68.4326 82.1501 68.5044C82.1748 68.5761 82.1845 68.6522 82.1786 68.7278C82.1811 68.8534 82.1611 68.9784 82.1197 69.0969C82.0953 69.1743 82.0645 69.2496 82.0277 69.322H80.3747V73.2339H79.1524V69.322Z\" fill=\"white\"/>\n<path d=\"M83.2021 73.2342V68.717C83.193 68.6374 83.204 68.5567 83.2343 68.4826C83.2645 68.4084 83.313 68.343 83.3751 68.2926C83.5137 68.2056 83.6756 68.1631 83.839 68.1708C83.9635 68.1704 84.0875 68.1878 84.2071 68.2225L84.4207 68.2889V70.0234H86.3755V68.717C86.3668 68.6371 86.3785 68.5562 86.4094 68.482C86.4403 68.4078 86.4894 68.3426 86.5522 68.2926C86.6913 68.2066 86.8528 68.1642 87.0161 68.1708C87.1407 68.1702 87.2647 68.1876 87.3843 68.2225L87.5978 68.2889V73.2342H86.3645V71.1896H84.417V73.2342H83.2021Z\" fill=\"white\"/>\n<path d=\"M89.144 68.8717C89.144 68.403 89.387 68.1705 89.8803 68.1705H92.4059C92.5792 68.1539 92.7521 68.2055 92.8881 68.3145C92.9386 68.3709 92.9773 68.4368 93.0019 68.5085C93.0266 68.5801 93.0367 68.6559 93.0317 68.7315C93.0323 68.8569 93.0124 68.9816 92.9728 69.1005C92.946 69.1777 92.914 69.2529 92.8771 69.3257H90.3884V70.1191H92.3101C92.4835 70.1025 92.6564 70.1541 92.7924 70.263C92.8429 70.3195 92.8816 70.3854 92.9062 70.457C92.9309 70.5286 92.941 70.6045 92.936 70.6801C92.9366 70.8055 92.9167 70.9302 92.8771 71.0491C92.852 71.1269 92.8199 71.2022 92.7814 71.2742H90.3884V72.0935H92.9949V73.2449H89.144V68.8717Z\" fill=\"white\"/>\n<path d=\"M96.5586 68.8717C96.5586 68.403 96.8016 68.1705 97.2949 68.1705H99.6437C99.817 68.1539 99.99 68.2055 100.126 68.3145C100.176 68.3709 100.215 68.4368 100.24 68.5085C100.264 68.5801 100.275 68.6559 100.27 68.7315C100.27 68.8569 100.25 68.9816 100.211 69.1005C100.184 69.1777 100.152 69.2529 100.115 69.3257H97.7956V70.1819H99.4523C99.6256 70.1652 99.7985 70.2168 99.9345 70.3258C99.985 70.3822 100.024 70.4481 100.048 70.5198C100.073 70.5914 100.083 70.6672 100.078 70.7428C100.079 70.8682 100.059 70.9929 100.019 71.1119C99.9924 71.189 99.9604 71.2642 99.9235 71.337H97.7956V73.2413H96.566L96.5586 68.8717Z\" fill=\"white\"/>\n<path d=\"M100.594 73.234C100.697 72.9584 100.819 72.6373 100.962 72.2707C101.109 71.9017 101.267 71.5142 101.437 71.1046C101.606 70.6949 101.783 70.2742 101.967 69.8461C102.151 69.418 102.335 69.0047 102.508 68.6061C102.575 68.4273 102.694 68.2729 102.85 68.1633C103.013 68.0723 103.198 68.0288 103.384 68.0378C103.467 68.0383 103.55 68.0482 103.631 68.0673C103.705 68.0846 103.778 68.108 103.848 68.1374C103.902 68.158 103.954 68.1839 104.003 68.2149L104.076 68.2592C104.183 68.4485 104.28 68.6431 104.367 68.8423C104.489 69.108 104.617 69.4069 104.761 69.7391C104.905 70.0712 105.052 70.4181 105.203 70.7835C105.354 71.1488 105.497 71.4921 105.626 71.8205C105.755 72.149 105.869 72.4368 105.969 72.6915C106.068 72.9461 106.134 73.1269 106.175 73.234H104.82C104.763 73.0913 104.705 72.9326 104.643 72.7579C104.581 72.5844 104.503 72.3888 104.408 72.1416H102.272C102.195 72.3482 102.125 72.5365 102.062 72.7099C102 72.8834 101.937 73.0568 101.867 73.234H100.594ZM103.995 71.0197C103.947 70.8868 103.888 70.7392 103.826 70.5731L103.642 70.0934C103.583 69.9384 103.528 69.7944 103.476 69.6653C103.425 69.5361 103.384 69.4365 103.355 69.3737C103.325 69.4438 103.285 69.5398 103.237 69.6653L103.075 70.086L102.891 70.5621L102.714 71.0197H103.995Z\" fill=\"white\"/>\n<path d=\"M107.471 71.7872C107.533 71.8319 107.6 71.8702 107.669 71.9016C107.796 71.9627 107.926 72.0145 108.06 72.0566C108.238 72.1147 108.42 72.1616 108.605 72.1968C108.825 72.2396 109.05 72.2606 109.275 72.2596C109.433 72.2624 109.591 72.2488 109.746 72.219C109.842 72.2006 109.935 72.1643 110.018 72.1119C110.074 72.0748 110.117 72.0222 110.143 71.9606C110.162 71.9048 110.172 71.8462 110.173 71.7872C110.173 71.7308 110.16 71.6752 110.135 71.6247C110.11 71.5742 110.074 71.53 110.029 71.4956C109.868 71.3951 109.685 71.3332 109.495 71.3148L108.759 71.1819C108.559 71.1471 108.363 71.0978 108.17 71.0343C107.981 70.9759 107.803 70.8861 107.644 70.7686C107.47 70.6434 107.331 70.4763 107.239 70.2828C107.147 70.0892 107.105 69.8755 107.117 69.6615C107.115 69.4423 107.158 69.2251 107.242 69.023C107.322 68.8225 107.448 68.6441 107.611 68.5027C107.802 68.3428 108.024 68.2235 108.262 68.1521C108.562 68.0611 108.873 68.0176 109.186 68.0229C109.41 68.0216 109.634 68.0351 109.856 68.0635C110.023 68.0845 110.188 68.1178 110.35 68.1631C110.464 68.1949 110.576 68.2343 110.685 68.2812C110.749 68.31 110.811 68.342 110.872 68.3772C110.962 68.4328 111.042 68.5012 111.112 68.5802C111.186 68.6647 111.227 68.7738 111.226 68.8865C111.226 68.9685 111.211 69.0499 111.182 69.1263C111.154 69.2001 111.118 69.2706 111.075 69.3367C111.034 69.3958 110.989 69.4514 110.939 69.5028C110.887 69.5508 110.847 69.584 110.813 69.6098C110.744 69.5485 110.671 69.493 110.593 69.4437C110.477 69.3744 110.353 69.3186 110.224 69.2777C110.071 69.2244 109.913 69.1825 109.753 69.1522C109.566 69.1175 109.376 69.1002 109.186 69.1005C109.033 69.0955 108.879 69.1116 108.73 69.1485C108.638 69.1698 108.551 69.21 108.476 69.2666C108.422 69.3057 108.383 69.3616 108.365 69.4253C108.349 69.4778 108.34 69.5325 108.339 69.5877C108.338 69.6302 108.345 69.6727 108.358 69.7131C108.374 69.76 108.405 69.8002 108.446 69.8275C108.507 69.8724 108.574 69.9073 108.645 69.9309C108.765 69.9722 108.888 70.003 109.013 70.0231L109.794 70.1781C110.044 70.2249 110.291 70.2928 110.53 70.3811C110.72 70.4527 110.898 70.5535 111.056 70.68C111.194 70.7963 111.304 70.9427 111.377 71.1081C111.452 71.2983 111.488 71.5016 111.483 71.706C111.493 71.9397 111.445 72.1721 111.344 72.3831C111.244 72.5941 111.093 72.7772 110.905 72.9165C110.423 73.2346 109.851 73.3874 109.275 73.3519C108.979 73.355 108.684 73.3278 108.395 73.2708C108.156 73.2238 107.921 73.1609 107.692 73.0825C107.524 73.0256 107.361 72.9578 107.202 72.8796C107.122 72.8454 107.046 72.8007 106.977 72.7467L107.471 71.7872Z\" fill=\"white\"/>\n<path d=\"M114.031 69.3222H112.853C112.68 69.3397 112.506 69.288 112.371 69.1783C112.32 69.1219 112.282 69.056 112.257 68.9843C112.232 68.9127 112.222 68.8369 112.227 68.7613C112.225 68.6357 112.245 68.5108 112.286 68.3922C112.309 68.3139 112.342 68.2384 112.382 68.1671H116.431C116.605 68.1505 116.778 68.2021 116.914 68.3111C116.964 68.3675 117.003 68.4334 117.027 68.505C117.052 68.5766 117.062 68.6525 117.057 68.7281C117.058 68.8535 117.038 68.9782 116.998 69.0971C116.971 69.1742 116.94 69.2495 116.903 69.3222H115.253V73.2342H114.024L114.031 69.3222Z\" fill=\"white\"/>\n<path d=\"M119.833 73.2341C119.936 72.9585 120.059 72.6375 120.201 72.2709C120.348 71.9018 120.507 71.5143 120.676 71.1047C120.845 70.6951 121.022 70.2743 121.206 69.8462C121.39 69.4182 121.574 69.0048 121.747 68.6062C121.814 68.4274 121.934 68.2731 122.09 68.1634C122.252 68.0719 122.437 68.0283 122.624 68.0379C122.707 68.0386 122.789 68.0485 122.87 68.0674C122.944 68.0853 123.017 68.1087 123.088 68.1376C123.141 68.1589 123.193 68.1848 123.242 68.2151L123.316 68.2594C123.423 68.4486 123.52 68.6433 123.607 68.8424C123.724 69.1082 123.857 69.4071 124.001 69.7392L124.442 70.7836C124.593 71.1527 124.733 71.4922 124.866 71.8207L125.208 72.6916L125.414 73.2341H124.067C124.013 73.0914 123.954 72.9327 123.89 72.758C123.824 72.5846 123.747 72.389 123.654 72.1417H121.512C121.431 72.3484 121.361 72.5366 121.302 72.7101C121.243 72.8835 121.173 73.057 121.107 73.2341H119.833ZM123.235 71.0198L123.065 70.5733C122.999 70.4072 122.94 70.2485 122.881 70.0935C122.822 69.9385 122.767 69.7946 122.716 69.6654C122.664 69.5363 122.624 69.4366 122.594 69.3739C122.565 69.444 122.524 69.5399 122.476 69.6654C122.429 69.7909 122.373 69.9311 122.314 70.0861L122.13 70.5622C122.064 70.7246 122.005 70.8759 121.954 71.0198H123.235Z\" fill=\"white\"/>\n<path d=\"M126.416 68.813C126.412 68.7041 126.432 68.5957 126.474 68.4956C126.507 68.4173 126.56 68.3496 126.629 68.3C126.694 68.2515 126.768 68.2175 126.846 68.2003C126.927 68.1812 127.01 68.1713 127.093 68.1708C127.234 68.1648 127.374 68.1887 127.505 68.2409C127.575 68.2693 127.642 68.3052 127.704 68.348C127.726 68.3738 127.781 68.4439 127.87 68.562L128.179 68.9975C128.297 69.1709 128.429 69.3665 128.569 69.5695L128.978 70.1821L129.346 70.7357L129.611 71.1453V68.717C129.602 68.6371 129.614 68.5562 129.645 68.482C129.676 68.4078 129.725 68.3426 129.788 68.2926C129.926 68.2056 130.088 68.1631 130.252 68.1708C130.376 68.1703 130.5 68.1877 130.62 68.2225L130.819 68.2889V73.2342H129.663L129.456 72.902L129.133 72.4001L128.739 71.7986C128.599 71.5845 128.459 71.3742 128.326 71.1675L127.958 70.5844L127.642 70.1194V73.2342H126.427L126.416 68.813Z\" fill=\"white\"/>\n<path d=\"M132.365 68.8278C132.363 68.7324 132.378 68.6374 132.409 68.5473C132.434 68.4754 132.473 68.409 132.523 68.3517C132.568 68.3009 132.623 68.2606 132.685 68.2336C132.746 68.2084 132.809 68.1899 132.873 68.1783C132.966 68.1586 133.102 68.1401 133.282 68.1229C133.462 68.1044 133.723 68.0971 134.073 68.0971C134.493 68.0914 134.911 68.1486 135.314 68.2668C135.661 68.3676 135.984 68.5384 136.264 68.7687C136.525 68.9909 136.733 69.2698 136.871 69.5843C137.019 69.9342 137.092 70.3115 137.085 70.6915C137.092 71.07 137.024 71.4463 136.886 71.7986C136.756 72.1151 136.553 72.3959 136.293 72.6179C136.006 72.8557 135.672 73.0304 135.314 73.1309C134.872 73.2559 134.414 73.3156 133.955 73.308C133.399 73.308 133.002 73.308 132.759 73.2748L132.365 73.2342V68.8278ZM133.595 72.1492L133.79 72.1677C133.888 72.1677 134.011 72.1677 134.158 72.1677C134.384 72.1695 134.609 72.141 134.828 72.0828C135.018 72.0336 135.196 71.9428 135.347 71.8171C135.494 71.693 135.609 71.5346 135.682 71.3558C135.764 71.1442 135.804 70.9185 135.8 70.6915C135.802 70.4882 135.769 70.2861 135.704 70.0936C135.643 69.9166 135.544 69.7553 135.413 69.6212C135.274 69.4822 135.105 69.3763 134.92 69.3112C134.694 69.2331 134.456 69.1956 134.217 69.2005C134.025 69.2005 133.878 69.2005 133.775 69.2005H133.598L133.595 72.1492Z\" fill=\"white\"/>\n<path d=\"M140.589 68.8717C140.589 68.403 140.832 68.1705 141.326 68.1705H143.844C144.017 68.1539 144.19 68.2055 144.326 68.3145C144.377 68.3709 144.415 68.4368 144.44 68.5085C144.465 68.5801 144.475 68.6559 144.47 68.7315C144.472 68.857 144.452 68.982 144.411 69.1005C144.386 69.1783 144.354 69.2536 144.315 69.3257H141.826V70.1191H143.748C143.921 70.1025 144.094 70.1541 144.23 70.263C144.281 70.3195 144.32 70.3854 144.344 70.457C144.369 70.5286 144.379 70.6045 144.374 70.6801C144.376 70.8056 144.356 70.9306 144.315 71.0491C144.29 71.1269 144.258 71.2022 144.219 71.2742H141.826V72.0935H144.433V73.2449H140.589V68.8717Z\" fill=\"white\"/>\n<path d=\"M145.342 73.2342C145.445 72.9586 145.568 72.6375 145.71 72.2709C145.858 71.9019 146.016 71.5144 146.185 71.1048C146.355 70.6951 146.531 70.2744 146.715 69.8463C146.899 69.4182 147.084 69.0049 147.257 68.6063C147.315 68.4252 147.435 68.2699 147.594 68.1664C147.754 68.0629 147.944 68.0176 148.133 68.038C148.216 68.0385 148.299 68.0484 148.379 68.0675C148.454 68.0848 148.526 68.1082 148.597 68.1376C148.651 68.1573 148.703 68.1833 148.751 68.2151L148.825 68.2594C148.933 68.4479 149.03 68.6426 149.116 68.8425C149.237 69.1082 149.37 69.4071 149.51 69.7393C149.65 70.0714 149.801 70.4183 149.951 70.7837C150.102 71.149 150.246 71.4923 150.375 71.8207C150.504 72.1492 150.622 72.437 150.717 72.6917C150.813 72.9463 150.887 73.1271 150.923 73.2342H149.569C149.517 73.0915 149.458 72.9328 149.392 72.7581C149.329 72.5846 149.252 72.389 149.156 72.1418H147.014C146.936 72.3484 146.866 72.5367 146.804 72.7101C146.741 72.8836 146.679 73.057 146.609 73.2342H145.342ZM148.744 71.0199C148.696 70.887 148.641 70.7394 148.575 70.5733L148.391 70.0936C148.332 69.9386 148.276 69.7946 148.225 69.6655C148.173 69.5363 148.133 69.4367 148.103 69.3739L147.986 69.6655L147.824 70.0862C147.765 70.2412 147.706 70.3999 147.639 70.5623L147.463 71.0199H148.744Z\" fill=\"white\"/>\n<path d=\"M152.219 71.7872C152.282 71.8319 152.348 71.8702 152.418 71.9016C152.544 71.9627 152.675 72.0145 152.808 72.0566C152.987 72.1143 153.169 72.1611 153.353 72.1968C153.574 72.2396 153.798 72.2606 154.023 72.2596C154.181 72.2624 154.339 72.2488 154.494 72.219C154.591 72.2006 154.684 72.1643 154.767 72.1119C154.822 72.0748 154.866 72.0222 154.892 71.9606C154.911 71.9048 154.921 71.8462 154.922 71.7872C154.921 71.7308 154.908 71.6752 154.883 71.6247C154.859 71.5742 154.823 71.53 154.778 71.4956C154.616 71.3951 154.434 71.3332 154.244 71.3148L153.508 71.1819C153.308 71.1471 153.111 71.0978 152.919 71.0343C152.73 70.9751 152.552 70.8853 152.392 70.7686C152.219 70.6434 152.079 70.4763 151.987 70.2828C151.895 70.0892 151.854 69.8755 151.866 69.6615C151.864 69.4423 151.907 69.2251 151.991 69.023C152.072 68.8233 152.198 68.6451 152.359 68.5027C152.551 68.3433 152.772 68.2241 153.011 68.1521C153.31 68.0611 153.622 68.0176 153.935 68.0229C154.159 68.0216 154.383 68.0351 154.605 68.0635C154.772 68.0841 154.937 68.1174 155.098 68.1631C155.212 68.1949 155.324 68.2343 155.433 68.2812C155.497 68.31 155.56 68.342 155.621 68.3772C155.71 68.4328 155.791 68.5012 155.86 68.5802C155.935 68.6647 155.975 68.7738 155.974 68.8865C155.975 68.9685 155.96 69.0499 155.93 69.1263C155.902 69.2001 155.866 69.2706 155.823 69.3367C155.783 69.3958 155.737 69.4514 155.687 69.5028C155.636 69.5508 155.595 69.584 155.562 69.6098C155.494 69.5476 155.42 69.492 155.341 69.4437C155.225 69.3744 155.102 69.3186 154.973 69.2777C154.819 69.2239 154.662 69.182 154.502 69.1522C154.315 69.1175 154.125 69.1002 153.935 69.1005C153.781 69.0955 153.628 69.1116 153.478 69.1485C153.386 69.1698 153.3 69.21 153.224 69.2666C153.172 69.3068 153.134 69.3622 153.114 69.4253C153.097 69.4778 153.089 69.5325 153.088 69.5877C153.087 69.6302 153.093 69.6727 153.107 69.7131C153.123 69.76 153.154 69.8002 153.195 69.8275C153.256 69.8717 153.323 69.9065 153.394 69.9309C153.508 69.9712 153.625 70.002 153.743 70.0231L154.524 70.1781C154.775 70.2249 155.021 70.2928 155.26 70.3811C155.45 70.4527 155.628 70.5535 155.787 70.68C155.925 70.7963 156.034 70.9427 156.107 71.1081C156.182 71.2983 156.218 71.5016 156.214 71.706C156.223 71.9397 156.175 72.1721 156.075 72.3831C155.974 72.5941 155.823 72.7772 155.636 72.9165C155.153 73.2346 154.581 73.3874 154.005 73.3519C153.71 73.3548 153.415 73.3276 153.125 73.2708C152.887 73.2233 152.652 73.1604 152.422 73.0825C152.254 73.0256 152.091 72.9578 151.932 72.8796C151.852 72.8443 151.777 72.7997 151.708 72.7467L152.219 71.7872Z\" fill=\"white\"/>\n<path d=\"M158.794 71.3669C158.68 71.2303 158.562 71.079 158.426 70.9166C158.29 70.7542 158.176 70.5955 158.058 70.4405L157.73 70.0088C157.631 69.8759 157.546 69.7652 157.484 69.6766C157.421 69.588 157.381 69.5327 157.318 69.4478C157.256 69.3642 157.2 69.2767 157.149 69.1858C157.099 69.0994 157.056 69.0093 157.02 68.9164C156.988 68.8389 156.971 68.7564 156.968 68.6728C156.964 68.6025 156.977 68.5322 157.006 68.468C157.035 68.4038 157.079 68.3474 157.134 68.3038C157.251 68.2205 157.392 68.1777 157.535 68.182C157.654 68.1791 157.772 68.1953 157.885 68.23C157.947 68.2492 158.007 68.2726 158.065 68.3001L158.353 68.7355L158.721 69.2854L159.089 69.8353C159.21 70.0124 159.317 70.1601 159.406 70.2782C159.494 70.1601 159.586 70.0309 159.682 69.9091C159.777 69.7873 159.873 69.6286 159.969 69.481L160.249 69.0455L160.503 68.647L160.609 68.4994C160.653 68.442 160.705 68.3922 160.764 68.3517C160.831 68.3047 160.904 68.2674 160.981 68.241C161.076 68.2097 161.176 68.1948 161.276 68.1967C161.36 68.1961 161.445 68.2073 161.526 68.23C161.598 68.2501 161.668 68.276 161.736 68.3074C161.789 68.332 161.84 68.3617 161.887 68.396L161.968 68.4624C161.927 68.5473 161.868 68.658 161.784 68.7983C161.699 68.9385 161.6 69.0972 161.486 69.2707L161.117 69.8279C160.985 70.0235 160.852 70.2191 160.72 70.4073C160.587 70.5955 160.458 70.7764 160.333 70.9461C160.208 71.1159 160.098 71.2635 160.006 71.3927V73.2601H158.776L158.794 71.3669Z\" fill=\"white\"/>\n<path d=\"M165.186 68.717C165.177 68.6371 165.188 68.5562 165.219 68.482C165.25 68.4078 165.299 68.3426 165.362 68.2926C165.501 68.2056 165.663 68.1631 165.826 68.1708C165.951 68.1704 166.075 68.1878 166.194 68.2225L166.404 68.2889V70.8722C166.402 71.0692 166.42 71.266 166.459 71.459C166.49 71.6132 166.552 71.7591 166.643 71.8871C166.731 72.0036 166.848 72.0941 166.982 72.1492C167.323 72.2673 167.694 72.2673 168.035 72.1492C168.169 72.0941 168.287 72.0036 168.374 71.8871C168.465 71.7591 168.527 71.6132 168.558 71.459C168.598 71.2611 168.616 71.0594 168.613 70.8575V68.717C168.604 68.6371 168.616 68.5562 168.647 68.482C168.678 68.4078 168.727 68.3426 168.79 68.2926C168.928 68.2056 169.09 68.1631 169.254 68.1708C169.378 68.1704 169.502 68.1878 169.622 68.2225L169.832 68.2889V70.9903C169.851 71.3074 169.807 71.6251 169.703 71.9252C169.6 72.2254 169.438 72.5021 169.228 72.7396C168.742 73.1448 168.13 73.3667 167.498 73.3667C166.865 73.3667 166.253 73.1448 165.767 72.7396C165.557 72.5021 165.395 72.2254 165.292 71.9252C165.188 71.6251 165.144 71.3074 165.163 70.9903L165.186 68.717Z\" fill=\"white\"/>\n<path d=\"M171.345 73.2342V68.717C171.336 68.6371 171.348 68.5562 171.379 68.482C171.409 68.4078 171.459 68.3426 171.521 68.2926C171.66 68.2056 171.822 68.1631 171.985 68.1708C172.11 68.1704 172.234 68.1878 172.353 68.2225L172.563 68.2889V73.2342H171.345Z\" fill=\"white\"/>\n<path d=\"M177.625 69.3222H176.447C176.274 69.3389 176.101 69.2873 175.965 69.1783C175.915 69.1219 175.876 69.056 175.851 68.9843C175.827 68.9127 175.816 68.8369 175.821 68.7613C175.819 68.6357 175.839 68.5108 175.88 68.3922C175.905 68.3145 175.938 68.2392 175.976 68.1671H180.026C180.199 68.1505 180.372 68.2021 180.508 68.3111C180.558 68.3675 180.597 68.4334 180.622 68.505C180.646 68.5766 180.657 68.6525 180.652 68.7281C180.654 68.8536 180.634 68.9786 180.593 69.0971C180.568 69.1749 180.535 69.2502 180.497 69.3222H178.848V73.2342H177.618L177.625 69.3222Z\" fill=\"white\"/>\n<path d=\"M181.318 70.7022C181.315 70.3497 181.384 70.0003 181.519 69.675C181.654 69.3496 181.854 69.0551 182.106 68.809C182.356 68.5659 182.651 68.3742 182.975 68.2443C183.665 67.9687 184.434 67.9687 185.125 68.2443C185.448 68.3742 185.743 68.5659 185.993 68.809C186.245 69.0557 186.444 69.3503 186.581 69.6754C186.717 70.0004 186.787 70.3496 186.787 70.7022C186.787 71.0549 186.717 71.4039 186.581 71.729C186.444 72.0541 186.245 72.3487 185.993 72.5954C185.743 72.8385 185.448 73.0302 185.125 73.1601C184.434 73.4357 183.665 73.4357 182.975 73.1601C182.651 73.0302 182.356 72.8385 182.106 72.5954C181.854 72.3493 181.654 72.0548 181.519 71.7294C181.384 71.4041 181.315 71.0547 181.318 70.7022ZM184.05 72.2153C184.249 72.218 184.447 72.1791 184.631 72.1009C184.804 72.026 184.959 71.9168 185.088 71.7798C185.217 71.6406 185.318 71.4777 185.386 71.3001C185.458 71.1092 185.495 70.9064 185.493 70.7022C185.494 70.4968 185.458 70.2929 185.386 70.1007C185.32 69.9224 185.218 69.7592 185.088 69.6209C184.959 69.485 184.803 69.377 184.631 69.3035C184.258 69.1509 183.841 69.1509 183.468 69.3035C183.296 69.377 183.14 69.485 183.011 69.6209C182.881 69.7592 182.78 69.9224 182.713 70.1007C182.641 70.2929 182.605 70.4968 182.606 70.7022C182.605 70.9064 182.641 71.1092 182.713 71.3001C182.781 71.4777 182.882 71.6406 183.011 71.7798C183.14 71.9168 183.296 72.026 183.468 72.1009C183.652 72.1791 183.85 72.218 184.05 72.2153Z\" fill=\"white\"/>\n<path d=\"M187.794 70.7022C187.791 70.3497 187.859 70.0003 187.995 69.675C188.13 69.3496 188.33 69.0551 188.582 68.809C188.832 68.5659 189.127 68.3742 189.45 68.2443C190.141 67.9687 190.91 67.9687 191.6 68.2443C191.924 68.3742 192.219 68.5659 192.469 68.809C192.721 69.0557 192.92 69.3503 193.057 69.6754C193.193 70.0004 193.263 70.3496 193.263 70.7022C193.263 71.0549 193.193 71.4039 193.057 71.729C192.92 72.0541 192.721 72.3487 192.469 72.5954C192.219 72.8385 191.924 73.0302 191.6 73.1601C190.91 73.4357 190.141 73.4357 189.45 73.1601C189.127 73.0302 188.832 72.8385 188.582 72.5954C188.33 72.3493 188.13 72.0548 187.995 71.7294C187.859 71.4041 187.791 71.0547 187.794 70.7022ZM190.525 72.2153C190.725 72.218 190.923 72.1791 191.107 72.1009C191.279 72.026 191.435 71.9168 191.564 71.7798C191.693 71.6406 191.794 71.4777 191.862 71.3001C191.934 71.1092 191.97 70.9064 191.968 70.7022C191.97 70.4968 191.934 70.2929 191.862 70.1007C191.795 69.9224 191.694 69.7592 191.564 69.6209C191.435 69.485 191.279 69.377 191.107 69.3035C190.734 69.1509 190.317 69.1509 189.944 69.3035C189.772 69.377 189.616 69.485 189.487 69.6209C189.357 69.7592 189.255 69.9224 189.189 70.1007C189.117 70.2929 189.081 70.4968 189.082 70.7022C189.08 70.9064 189.117 71.1092 189.189 71.3001C189.257 71.4777 189.358 71.6406 189.487 71.7798C189.616 71.9168 189.771 72.026 189.944 72.1009C190.128 72.1791 190.326 72.218 190.525 72.2153Z\" fill=\"white\"/>\n<path d=\"M194.527 73.2342V68.717C194.518 68.6371 194.53 68.5562 194.561 68.482C194.592 68.4078 194.641 68.3426 194.704 68.2926C194.842 68.2056 195.004 68.1631 195.168 68.1708C195.292 68.1704 195.416 68.1878 195.536 68.2225L195.746 68.2889V72.0827H198.08V73.2342H194.527Z\" fill=\"white\"/>\n<path d=\"M199.081 73.2342V68.7171C199.072 68.6372 199.084 68.5563 199.115 68.4821C199.146 68.4079 199.195 68.3427 199.258 68.2927C199.397 68.2057 199.558 68.1632 199.722 68.1709C199.846 68.1705 199.97 68.1879 200.09 68.2226L200.3 68.289V70.1675L200.668 69.7504L201.091 69.3002C201.235 69.1452 201.375 69.0013 201.511 68.8647L201.879 68.4957C201.975 68.4113 202.079 68.336 202.188 68.2706C202.323 68.1955 202.476 68.1586 202.63 68.1635C202.735 68.1629 202.839 68.1765 202.939 68.2041C203.023 68.2261 203.103 68.2583 203.179 68.3001C203.238 68.3316 203.294 68.37 203.344 68.4145C203.379 68.4436 203.411 68.4757 203.44 68.5104C203.418 68.5363 203.359 68.5916 203.271 68.6802L202.958 68.9976L202.564 69.3998C202.424 69.5438 202.284 69.6877 202.144 69.8242L201.776 70.2191L201.5 70.5217C201.706 70.7432 201.905 70.9609 202.089 71.175C202.273 71.389 202.457 71.6067 202.63 71.8282C202.803 72.0496 202.976 72.271 203.146 72.4998C203.315 72.7286 203.488 72.9722 203.665 73.2379H202.155C202.03 73.0645 201.887 72.8689 201.728 72.6807C201.57 72.4924 201.412 72.2821 201.246 72.0828C201.08 71.8835 200.918 71.6916 200.749 71.5071C200.58 71.3226 200.432 71.1676 200.292 71.0273V73.2416L199.081 73.2342Z\" fill=\"white\"/>\n<path d=\"M204.692 73.2342V68.717C204.683 68.6371 204.695 68.5562 204.726 68.482C204.757 68.4078 204.806 68.3426 204.868 68.2926C205.007 68.2056 205.169 68.1631 205.332 68.1708C205.457 68.1704 205.581 68.1878 205.701 68.2225L205.91 68.2889V73.2342H204.692Z\" fill=\"white\"/>\n<path d=\"M208.705 69.322H207.527C207.353 69.3387 207.18 69.2871 207.044 69.1781C206.994 69.1217 206.955 69.0557 206.93 68.9841C206.906 68.9125 206.896 68.8367 206.901 68.7611C206.9 68.6357 206.92 68.511 206.96 68.392C206.986 68.3149 207.018 68.2397 207.055 68.1669H211.105C211.278 68.1494 211.452 68.2012 211.587 68.3108C211.638 68.3672 211.676 68.4332 211.701 68.5048C211.726 68.5764 211.736 68.6523 211.731 68.7278C211.733 68.8534 211.713 68.9784 211.672 69.0969C211.648 69.1753 211.616 69.2508 211.576 69.322H209.927V73.2339H208.705V69.322Z\" fill=\"white\"/>\n<path d=\"M104.12 44.3745C102.132 42.8835 99.3479 41.8994 95.7671 41.4221L89.1588 40.5954C87.3867 40.3814 86.027 39.9816 85.0797 39.396C84.6278 39.1388 84.2556 38.7614 84.0042 38.3056C83.7528 37.8497 83.6319 37.3331 83.6549 36.8127C83.6549 34.1678 86.1596 32.8454 91.1689 32.8454C92.7741 32.827 94.3773 32.9679 95.9548 33.2661C97.1649 33.503 98.356 33.8287 99.5185 34.2404C100.34 34.5315 101.132 34.9023 101.882 35.3476C102.283 35.5849 102.662 35.8565 103.016 36.1595C103.227 36.0662 103.423 35.9418 103.598 35.7904C103.875 35.5781 104.122 35.3299 104.334 35.0523C104.584 34.7295 104.801 34.382 104.982 34.0153C105.184 33.5893 105.284 33.1215 105.273 32.6498C105.273 31.0531 104.032 29.7553 101.551 28.7564C99.0694 27.7575 95.583 27.2654 91.0915 27.2802C88.7875 27.2351 86.4883 27.5083 84.2587 28.0921C82.6024 28.524 81.0498 29.2858 79.6936 30.3322C78.5991 31.198 77.7259 32.3125 77.146 33.5835C76.6062 34.8229 76.3304 36.1615 76.3361 37.5139C76.2986 38.9381 76.5989 40.3509 77.2123 41.6361C77.7881 42.7739 78.6199 43.7619 79.6421 44.5221C80.7687 45.3454 82.0275 45.9695 83.3641 46.3673C84.9149 46.8419 86.5098 47.1572 88.1243 47.3084L93.1753 47.8915C95.5069 48.1375 97.1255 48.5816 98.0312 49.2238C98.4751 49.5276 98.8335 49.9407 99.0721 50.4235C99.3106 50.9063 99.4213 51.4425 99.3934 51.9805C99.3934 53.41 98.6804 54.4163 97.2544 54.9994C95.8284 55.5824 93.9877 55.8752 91.7321 55.8777C90.1285 55.8861 88.5272 55.7552 86.9462 55.4865C85.6206 55.2626 84.3119 54.9482 83.029 54.5454C82.0495 54.2413 81.0872 53.884 80.1464 53.4752C79.583 53.2437 79.042 52.9608 78.5302 52.6301L75.7433 57.6307C76.2736 58.0059 76.8367 58.332 77.4258 58.6049C78.5262 59.1296 79.6631 59.5737 80.8275 59.9335C82.4496 60.4499 84.1037 60.8592 85.7791 61.1588C87.8741 61.5191 89.9969 61.692 92.1224 61.6754C96.9599 61.6754 100.662 60.7024 103.229 58.7563C104.458 57.872 105.451 56.6996 106.123 55.3415C106.795 53.9834 107.125 52.481 107.084 50.9657C107.094 48.065 106.106 45.8679 104.12 44.3745Z\" fill=\"white\"/>\n<path d=\"M119.156 12.917C117.904 12.917 116.932 13.1987 116.24 13.7621C115.551 14.3231 115.205 15.1903 115.205 16.3454V60.7677H122.594V13.6551C122.376 13.569 121.945 13.4287 121.302 13.2344C120.606 13.0235 119.883 12.9166 119.156 12.917Z\" fill=\"white\"/>\n<path d=\"M136.216 13.5735C135.627 13.5666 135.043 13.6777 134.498 13.9001C133.953 14.1226 133.458 14.452 133.041 14.8691C132.625 15.2861 132.296 15.7823 132.074 16.3286C131.851 16.8748 131.74 17.4601 131.746 18.0501C131.729 18.649 131.832 19.2453 132.049 19.8037C132.266 20.3621 132.592 20.8712 133.009 21.3009C133.425 21.7306 133.923 22.0722 134.474 22.3055C135.025 22.5387 135.616 22.6589 136.214 22.6589C136.812 22.6589 137.403 22.5387 137.954 22.3055C138.504 22.0722 139.003 21.7306 139.419 21.3009C139.836 20.8712 140.162 20.3621 140.379 19.8037C140.596 19.2453 140.699 18.649 140.681 18.0501C140.688 17.4604 140.577 16.8754 140.355 16.3294C140.132 15.7834 139.804 15.2874 139.388 14.8704C138.972 14.4534 138.477 14.1239 137.932 13.9012C137.388 13.6785 136.804 13.5671 136.216 13.5735Z\" fill=\"white\"/>\n<path d=\"M136.459 28.1807C135.207 28.1807 134.236 28.4501 133.547 28.9889C132.857 29.5277 132.511 30.4048 132.508 31.6202V60.775H139.897V28.893C139.684 28.8081 139.249 28.6679 138.605 28.4723C137.908 28.2701 137.185 28.1719 136.459 28.1807Z\" fill=\"white\"/>\n<path d=\"M176.602 30.9375C175.124 29.7079 173.419 28.7813 171.584 28.2102C169.53 27.5661 167.389 27.2485 165.237 27.2692C163.096 27.2468 160.965 27.5644 158.923 28.2102C157.101 28.7865 155.408 29.7128 153.939 30.9375C152.529 32.1269 151.398 33.6123 150.625 35.2886C149.8 37.1115 149.39 39.0958 149.425 41.0974V60.7676H156.788V42.9795C156.788 40.0271 157.49 37.7883 158.894 36.2629C160.298 34.7375 162.412 33.9674 165.237 33.9526C167.998 33.9526 170.092 34.7215 171.518 36.2592C172.944 37.7969 173.657 40.0358 173.657 42.9758V60.7639H181.038V41.0937C181.073 39.096 180.676 37.1145 179.875 35.2849C179.119 33.6103 178.001 32.1252 176.602 30.9375Z\" fill=\"white\"/>\n<path d=\"M208.171 53.4308C207.446 53.9013 206.663 54.2736 205.84 54.5379C204.718 54.9365 203.534 55.1351 202.343 55.1247C201.59 55.1269 200.84 55.0389 200.108 54.8627C199.423 54.7033 198.782 54.3932 198.231 53.9548C197.672 53.4974 197.238 52.9058 196.968 52.235C196.637 51.3537 196.483 50.4156 196.515 49.4746V34.9968H208.948C209.15 34.5636 209.327 34.1187 209.478 33.6646C209.699 33.0061 209.809 32.3149 209.802 31.62C209.802 30.4932 209.51 29.6321 208.925 29.0367C208.341 28.4413 207.339 28.1498 205.918 28.1621H196.515V19.674C196.298 19.5891 195.856 19.4489 195.186 19.2533C194.481 19.0535 193.751 18.9554 193.018 18.9617C191.81 18.9617 190.86 19.2324 190.168 19.7736C189.476 20.3124 189.134 21.1908 189.134 22.4012V50.9655C189.134 54.1639 190.104 56.7473 192.046 58.7155C193.987 60.6838 196.859 61.6679 200.661 61.6679C201.897 61.6721 203.131 61.5485 204.342 61.2989C205.378 61.085 206.396 60.7914 207.387 60.4205C208.186 60.1245 208.957 59.7538 209.688 59.3134C210.31 58.9222 210.792 58.6011 211.142 58.3391L208.171 53.4308Z\" fill=\"white\"/>\n<defs>\n<radialGradient id=\"paint0_radial_1014_7422\" cx=\"0\" cy=\"0\" r=\"1\" gradientUnits=\"userSpaceOnUse\" gradientTransform=\"translate(20.1931 28.7748) scale(103.145 103.396)\">\n<stop offset=\"0.1\" stop-color=\"#2C2F36\"/>\n<stop offset=\"0.33\" stop-color=\"#202328\"/>\n<stop offset=\"0.94\" stop-color=\"#040708\"/>\n</radialGradient>\n</defs>\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = b"<svg width=\"214\" height=\"93\" viewBox=\"0 0 214 93\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M57.4868 3.875H10.5827C6.53517 3.875 3.25397 7.17956 3.25397 11.256V81.375C3.25397 85.4514 6.53517 88.756 10.5827 88.756H57.4868C61.5344 88.756 64.8156 85.4514 64.8156 81.375V11.256C64.8156 7.17956 61.5344 3.875 57.4868 3.875Z\" fill=\"url(#paint0_radial_1014_7487)\"/>\n<path d=\"M19.6411 77.1089L55.8562 51.855C55.8562 51.855 57.4905 50.9102 57.4905 49.4193C57.4905 47.4338 55.4238 46.7879 55.4238 46.7879L35.5005 38.8497C34.7897 38.5693 33.8113 39.359 34.7274 40.3518L41.3232 47.0168C41.3232 47.0168 43.1554 48.8251 43.1554 50.0134C43.1554 51.2018 42.0341 52.2609 42.0341 52.2609L18.0288 75.581C17.175 76.4114 18.3219 77.9171 19.6411 77.1089Z\" fill=\"#0025FF\"/>\n<path d=\"M48.8939 13.0752L12.6787 38.3291C12.6787 38.3291 11.0444 39.2739 11.0444 40.7648C11.0444 42.7503 13.1111 43.3961 13.1111 43.3961L33.0344 51.3343C33.7453 51.6111 34.7274 50.8251 33.8076 49.8323L27.208 43.1784C27.208 43.1784 25.3758 41.3701 25.3758 40.178C25.3758 38.986 26.5008 37.9305 26.5008 37.9305L50.5209 14.6104C51.36 13.769 50.202 12.2633 48.8939 13.0752Z\" fill=\"#0025FF\"/>\n<path d=\"M78.0514 69.012H76.8788C76.7063 69.0286 76.5341 68.977 76.3987 68.868C76.3485 68.8116 76.31 68.7457 76.2855 68.6741C76.2609 68.6024 76.2508 68.5266 76.2558 68.451C76.2552 68.3256 76.275 68.2009 76.3144 68.082C76.3404 68.0046 76.3722 67.9293 76.4097 67.8568H80.4405C80.6131 67.8394 80.7856 67.8911 80.9206 68.0008C80.9712 68.0569 81.0099 68.1229 81.0344 68.1945C81.059 68.2662 81.0689 68.3422 81.0635 68.4178C81.0659 68.5433 81.046 68.6683 81.0049 68.7868C80.9815 68.8652 80.9495 68.9407 80.9096 69.012H79.2679V72.9239H78.0514V69.012Z\" fill=\"black\"/>\n<path d=\"M82.0492 72.9129V68.3957C82.0405 68.3158 82.0521 68.2349 82.0829 68.1607C82.1136 68.0865 82.1626 68.0213 82.2251 67.9713C82.3631 67.8843 82.5242 67.8418 82.6868 67.8495C82.8109 67.8474 82.9345 67.8649 83.0532 67.9012L83.2621 67.9676V69.7022H85.2116V68.3957C85.2029 68.3158 85.2145 68.2349 85.2453 68.1607C85.276 68.0865 85.3249 68.0213 85.3875 67.9713C85.5254 67.8843 85.6865 67.8418 85.8492 67.8495C85.9732 67.8474 86.0969 67.8649 86.2156 67.9012L86.4245 67.9676V72.9129H85.2006V70.8573H83.2511V72.9018L82.0492 72.9129Z\" fill=\"black\"/>\n<path d=\"M87.9818 68.5504C87.9818 68.0818 88.2237 67.8493 88.7147 67.8493H91.2211C91.3937 67.8326 91.5658 67.8842 91.7012 67.9932C91.7518 68.0494 91.7905 68.1153 91.8151 68.187C91.8396 68.2586 91.8495 68.3346 91.8441 68.4102C91.8447 68.5356 91.8249 68.6603 91.7855 68.7793C91.7588 68.8564 91.727 68.9316 91.6902 69.0044H89.2131V69.7941H91.1259C91.2984 69.7775 91.4705 69.8291 91.6059 69.9381C91.6565 69.9942 91.6952 70.0602 91.7198 70.1318C91.7443 70.2035 91.7542 70.2795 91.7488 70.3551C91.7494 70.4805 91.7296 70.6052 91.6902 70.7241C91.6635 70.8013 91.6317 70.8765 91.5949 70.9493H89.2131V71.7685H91.8074V72.92H87.9818V68.5504Z\" fill=\"black\"/>\n<path d=\"M95.3619 68.5504C95.3619 68.0818 95.6037 67.8493 96.0948 67.8493H98.4327C98.6052 67.8326 98.7773 67.8842 98.9127 67.9932C98.9633 68.0494 99.002 68.1153 99.0266 68.187C99.0511 68.2586 99.061 68.3346 99.0556 68.4102C99.0562 68.5356 99.0364 68.6603 98.997 68.7793C98.9703 68.8564 98.9385 68.9316 98.9017 69.0044H96.5931V69.8606H98.2421C98.4146 69.8439 98.5868 69.8955 98.7222 70.0045C98.7727 70.0607 98.8115 70.1266 98.836 70.1983C98.8606 70.2699 98.8705 70.3459 98.8651 70.4215C98.8657 70.5469 98.8459 70.6716 98.8064 70.7906C98.7798 70.8677 98.7479 70.9429 98.7112 71.0157H96.5931V72.92H95.3692L95.3619 68.5504Z\" fill=\"black\"/>\n<path d=\"M99.3781 72.9127C99.4807 72.6371 99.6028 72.3161 99.7445 71.9495C99.8911 71.5804 100.049 71.1929 100.217 70.7833C100.386 70.3736 100.562 69.9529 100.745 69.5248C100.928 69.0967 101.111 68.6834 101.284 68.2848C101.348 68.1048 101.467 67.9499 101.624 67.842C101.786 67.751 101.97 67.7075 102.156 67.7165C102.238 67.7152 102.321 67.7251 102.401 67.746C102.476 67.7616 102.548 67.7851 102.617 67.8161C102.671 67.8374 102.722 67.8634 102.771 67.8936L102.845 67.9379C102.951 68.1272 103.047 68.3218 103.134 68.521C103.255 68.7867 103.383 69.0857 103.526 69.4178C103.669 69.7499 103.816 70.0968 103.966 70.4622C104.116 70.8276 104.259 71.1708 104.387 71.4992L104.728 72.3702C104.827 72.6211 104.893 72.802 104.933 72.9127H103.592C103.538 72.77 103.48 72.6113 103.416 72.4366C103.354 72.2595 103.273 72.0676 103.182 71.8203H101.049C100.972 72.027 100.902 72.2152 100.84 72.3886C100.778 72.5621 100.716 72.7355 100.646 72.9127H99.3781ZM102.764 70.6984C102.716 70.5655 102.658 70.4142 102.595 70.2518L102.412 69.7721C102.354 69.6171 102.299 69.4732 102.247 69.344C102.196 69.2148 102.156 69.1152 102.126 69.0524C102.097 69.1189 102.057 69.2185 102.009 69.344L101.848 69.7647L101.665 70.2408L101.489 70.6984H102.764Z\" fill=\"black\"/>\n<path d=\"M106.219 71.466C106.282 71.5093 106.349 71.5476 106.417 71.5805C106.543 71.641 106.673 71.6928 106.806 71.7355C106.984 71.7914 107.165 71.8382 107.348 71.8757C107.568 71.9182 107.791 71.9392 108.015 71.9384C108.172 71.9415 108.33 71.9279 108.484 71.8978C108.58 71.8784 108.672 71.8422 108.755 71.7908C108.813 71.756 108.856 71.7026 108.88 71.6395C108.9 71.584 108.91 71.5252 108.909 71.466C108.909 71.4097 108.896 71.3541 108.871 71.3036C108.847 71.253 108.811 71.2089 108.766 71.1745C108.606 71.0734 108.424 71.0114 108.235 70.9937L107.502 70.8608C107.304 70.8234 107.108 70.7741 106.916 70.7132C106.729 70.6509 106.553 70.5614 106.392 70.4475C106.24 70.3301 106.115 70.1817 106.025 70.012C105.923 69.8087 105.873 69.5829 105.879 69.3551C105.878 69.1361 105.921 68.9192 106.003 68.7166C106.085 68.5176 106.21 68.3397 106.37 68.1963C106.561 68.0377 106.781 67.9185 107.018 67.8457C107.316 67.7533 107.627 67.7096 107.938 67.7165C108.161 67.7132 108.384 67.7268 108.605 67.7571C108.771 67.7777 108.935 67.8111 109.096 67.8568C109.211 67.8856 109.322 67.9252 109.429 67.9749C109.495 68.0005 109.557 68.0326 109.616 68.0708C109.707 68.1233 109.788 68.1921 109.855 68.2738C109.93 68.3574 109.971 68.4671 109.968 68.5801C109.968 68.6621 109.953 68.7434 109.924 68.82C109.896 68.8936 109.861 68.9641 109.818 69.0303C109.78 69.0912 109.734 69.1469 109.682 69.1964C109.643 69.2347 109.601 69.2704 109.558 69.3034C109.492 69.2381 109.418 69.1823 109.338 69.1374C109.223 69.0665 109.1 69.0107 108.971 68.9713C108.819 68.9158 108.662 68.8739 108.502 68.8458C108.317 68.8092 108.127 68.7918 107.938 68.7941C107.785 68.7893 107.632 68.8055 107.484 68.8421C107.392 68.8634 107.306 68.9036 107.231 68.9602C107.179 69.0008 107.141 69.0561 107.121 69.1189C107.106 69.1718 107.097 69.2263 107.095 69.2813C107.096 69.3238 107.102 69.366 107.114 69.4068C107.133 69.4516 107.163 69.4909 107.202 69.5212C107.262 69.5646 107.329 69.5993 107.399 69.6245C107.513 69.6626 107.63 69.6934 107.748 69.7168L108.524 69.8718C108.775 69.9157 109.02 69.9837 109.257 70.0747C109.447 70.1446 109.624 70.2456 109.781 70.3737C109.922 70.4871 110.031 70.6343 110.1 70.8018C110.176 70.9915 110.212 71.1951 110.206 71.3996C110.216 71.6334 110.169 71.8661 110.069 72.0771C109.968 72.2882 109.818 72.4712 109.631 72.6101C109.151 72.9278 108.581 73.0805 108.008 73.0456C107.714 73.0464 107.421 73.0192 107.132 72.9644C106.895 72.9165 106.661 72.8536 106.432 72.7762C106.266 72.717 106.104 72.6492 105.945 72.5732C105.867 72.5349 105.792 72.4905 105.721 72.4403L106.219 71.466Z\" fill=\"black\"/>\n<path d=\"M112.753 69.0117H111.58C111.408 69.0292 111.235 68.9774 111.1 68.8678C111.05 68.8114 111.012 68.7454 110.987 68.6738C110.963 68.6022 110.953 68.5263 110.958 68.4507C110.955 68.3252 110.975 68.2002 111.016 68.0817C111.039 68.0031 111.071 67.9275 111.111 67.8566H115.142C115.315 67.8399 115.487 67.8915 115.622 68.0005C115.673 68.0567 115.712 68.1226 115.736 68.1943C115.761 68.266 115.771 68.3419 115.765 68.4175C115.766 68.543 115.746 68.6676 115.707 68.7866C115.68 68.8637 115.648 68.9389 115.611 69.0117H113.962V72.9236H112.753V69.0117Z\" fill=\"black\"/>\n<path d=\"M118.524 72.9127C118.627 72.6371 118.749 72.3161 118.891 71.9495C119.037 71.5804 119.195 71.1929 119.364 70.7833C119.532 70.3736 119.708 69.9529 119.891 69.5248C120.075 69.0967 120.258 68.6834 120.43 68.2848C120.497 68.1062 120.616 67.952 120.771 67.842C120.933 67.751 121.117 67.7075 121.302 67.7165C121.385 67.7149 121.467 67.7249 121.548 67.746C121.622 67.7616 121.694 67.7851 121.764 67.8161C121.818 67.8358 121.87 67.8618 121.918 67.8936L121.991 67.9379C122.099 68.1264 122.195 68.3211 122.28 68.521C122.403 68.7867 122.535 69.0857 122.676 69.4178C122.815 69.7499 122.962 70.0968 123.112 70.4622C123.263 70.8276 123.405 71.1708 123.534 71.4992C123.662 71.8277 123.779 72.1155 123.874 72.3702C123.97 72.6248 124.043 72.802 124.08 72.9127H122.757C122.706 72.77 122.647 72.6113 122.581 72.4366C122.519 72.2595 122.442 72.0676 122.35 71.8203H120.192C120.115 72.027 120.045 72.2152 119.983 72.3886C119.921 72.5621 119.858 72.7355 119.789 72.9127H118.524ZM121.91 70.6984C121.863 70.5655 121.808 70.4142 121.742 70.2518L121.559 69.7721C121.5 69.6171 121.445 69.4732 121.394 69.344C121.342 69.2148 121.302 69.1152 121.273 69.0524C121.243 69.1189 121.207 69.2185 121.156 69.344L120.994 69.7647C120.936 69.9197 120.877 70.0784 120.811 70.2408C120.745 70.4032 120.69 70.5545 120.639 70.6984H121.91Z\" fill=\"black\"/>\n<path d=\"M125.08 68.4918C125.076 68.3833 125.095 68.2752 125.135 68.1744C125.169 68.096 125.223 68.0284 125.293 67.9788C125.356 67.9296 125.43 67.8955 125.509 67.8792C125.588 67.8583 125.669 67.8483 125.751 67.8496C125.892 67.8437 126.033 67.8676 126.165 67.9198C126.235 67.9474 126.301 67.9833 126.363 68.0268C126.385 68.0489 126.44 68.1227 126.524 68.2408C126.608 68.3589 126.714 68.5029 126.835 68.6763C126.956 68.8498 127.081 69.0454 127.22 69.2483L127.63 69.861C127.759 70.0602 127.883 70.23 127.997 70.4145C128.111 70.599 128.195 70.7208 128.257 70.8242V68.3958C128.248 68.3159 128.26 68.235 128.291 68.1608C128.322 68.0866 128.37 68.0214 128.433 67.9714C128.572 67.8839 128.735 67.8414 128.898 67.8496C129.022 67.8467 129.146 67.8642 129.265 67.9013L129.463 67.9677V72.913H128.312C128.268 72.8392 128.202 72.7284 128.107 72.5808C128.012 72.4332 127.909 72.2634 127.784 72.0789L127.396 71.4774C127.253 71.2633 127.117 71.053 126.986 70.8463C126.854 70.6396 126.722 70.444 126.597 70.2632C126.473 70.0824 126.374 69.9237 126.282 69.7982V72.913H125.076L125.08 68.4918Z\" fill=\"black\"/>\n<path d=\"M131.002 68.5065C130.999 68.4111 131.014 68.316 131.046 68.226C131.07 68.1537 131.109 68.0872 131.159 68.0304C131.204 67.9796 131.259 67.9393 131.321 67.9123C131.379 67.8838 131.443 67.8651 131.507 67.857C131.641 67.8288 131.777 67.8103 131.914 67.8016C132.094 67.7832 132.354 67.7758 132.702 67.7758C133.12 67.7701 133.536 67.8273 133.937 67.9455C134.283 68.0448 134.605 68.2157 134.882 68.4474C135.143 68.6689 135.35 68.948 135.487 69.263C135.635 69.6128 135.707 69.9901 135.699 70.3702C135.706 70.7487 135.639 71.125 135.502 71.4773C135.372 71.7938 135.17 72.0746 134.912 72.2966C134.625 72.533 134.293 72.7076 133.937 72.8096C133.497 72.9347 133.042 72.9943 132.585 72.9867C132.031 72.9867 131.636 72.9867 131.394 72.9535L131.002 72.9129V68.5065ZM132.226 71.8279H132.42C132.518 71.8279 132.64 71.8279 132.786 71.8279C133.011 71.8298 133.236 71.8012 133.453 71.743C133.642 71.6908 133.817 71.6004 133.97 71.4773C134.116 71.3524 134.23 71.1943 134.303 71.016C134.385 70.8044 134.425 70.5788 134.421 70.3517C134.422 70.1485 134.39 69.9464 134.325 69.7539C134.264 69.5771 134.165 69.416 134.036 69.2815C133.897 69.1425 133.729 69.0366 133.545 68.9715C133.32 68.8933 133.083 68.8559 132.845 68.8608C132.654 68.8608 132.508 68.8608 132.405 68.8608C132.347 68.8555 132.288 68.8555 132.229 68.8608L132.226 71.8279Z\" fill=\"black\"/>\n<path d=\"M139.188 68.5504C139.188 68.0818 139.43 67.8493 139.921 67.8493H142.427C142.6 67.8326 142.772 67.8842 142.907 67.9932C142.958 68.0494 142.997 68.1153 143.021 68.187C143.046 68.2586 143.056 68.3346 143.05 68.4102C143.051 68.5356 143.031 68.6603 142.992 68.7793C142.967 68.857 142.935 68.9324 142.896 69.0044H140.419V69.7941H142.332C142.504 69.7775 142.677 69.8291 142.812 69.9381C142.863 69.9942 142.901 70.0602 142.926 70.1318C142.95 70.2035 142.96 70.2795 142.955 70.3551C142.957 70.4806 142.937 70.6056 142.896 70.7241C142.871 70.8019 142.839 70.8772 142.801 70.9493H140.419V71.7685H143.014V72.92H139.188V68.5504Z\" fill=\"black\"/>\n<path d=\"M143.919 72.9127C144.021 72.6371 144.143 72.3161 144.285 71.9495C144.432 71.5804 144.589 71.1929 144.758 70.7833C144.926 70.3736 145.102 69.9529 145.285 69.5248C145.469 69.0967 145.652 68.6834 145.824 68.2848C145.891 68.1062 146.01 67.952 146.165 67.842C146.327 67.751 146.511 67.7075 146.696 67.7165C146.779 67.7152 146.862 67.7251 146.942 67.746C147.016 67.7616 147.089 67.7851 147.158 67.8161C147.212 67.8358 147.264 67.8618 147.312 67.8936L147.385 67.9379C147.493 68.1264 147.59 68.3211 147.675 68.521C147.796 68.7867 147.927 69.0857 148.067 69.4178C148.206 69.7499 148.356 70.0968 148.506 70.4622C148.657 70.8276 148.8 71.1708 148.928 71.4992C149.056 71.8277 149.173 72.1155 149.269 72.3702C149.364 72.6248 149.437 72.802 149.474 72.9127H148.125C148.074 72.77 148.015 72.6113 147.949 72.4366C147.887 72.2595 147.81 72.0676 147.715 71.8203H145.582C145.505 72.027 145.436 72.2152 145.373 72.3886C145.311 72.5621 145.249 72.7355 145.179 72.9127H143.919ZM147.305 70.6984C147.257 70.5655 147.202 70.4142 147.136 70.2518L146.953 69.7721C146.894 69.6171 146.839 69.4732 146.788 69.344C146.737 69.2148 146.696 69.1152 146.667 69.0524C146.638 69.1189 146.601 69.2185 146.55 69.344L146.388 69.7647C146.33 69.9197 146.271 70.0784 146.205 70.2408L146.029 70.6984H147.305Z\" fill=\"black\"/>\n<path d=\"M150.764 71.4661C150.827 71.5094 150.893 71.5476 150.962 71.5805C151.087 71.6417 151.217 71.6935 151.35 71.7355C151.528 71.7919 151.709 71.8388 151.892 71.8757C152.112 71.9185 152.335 71.9395 152.559 71.9385C152.717 71.9413 152.874 71.9277 153.028 71.8979C153.124 71.8778 153.216 71.8417 153.299 71.7909C153.355 71.7537 153.398 71.7011 153.424 71.6396C153.443 71.5836 153.453 71.5251 153.453 71.4661C153.453 71.4097 153.44 71.3542 153.416 71.3036C153.391 71.2531 153.355 71.2089 153.31 71.1746C153.149 71.074 152.968 71.0122 152.779 70.9937L152.046 70.8609C151.848 70.8243 151.652 70.775 151.46 70.7132C151.273 70.6518 151.096 70.5622 150.936 70.4475C150.784 70.3312 150.659 70.1825 150.57 70.0121C150.465 69.8094 150.415 69.5832 150.423 69.3552C150.421 69.136 150.463 68.9187 150.548 68.7167C150.627 68.5162 150.752 68.3377 150.914 68.1963C151.104 68.0365 151.325 67.9172 151.563 67.8457C151.86 67.753 152.171 67.7094 152.482 67.7166C152.705 67.7134 152.928 67.727 153.149 67.7572C153.315 67.7782 153.479 67.8115 153.64 67.8568C153.754 67.8869 153.866 67.9264 153.974 67.9749C154.038 68.0022 154.101 68.0342 154.161 68.0709C154.25 68.1253 154.331 68.1938 154.399 68.2738C154.473 68.3584 154.513 68.4674 154.512 68.5802C154.512 68.6621 154.497 68.7434 154.468 68.82C154.439 68.893 154.403 68.9634 154.362 69.0304C154.322 69.0895 154.276 69.1451 154.227 69.1965L154.102 69.3035C154.035 69.2399 153.961 69.1842 153.882 69.1374C153.767 69.0681 153.644 69.0123 153.516 68.9713C153.363 68.9164 153.206 68.8744 153.047 68.8459C152.861 68.8094 152.672 68.7921 152.482 68.7942C152.329 68.7892 152.177 68.8053 152.028 68.8422C151.936 68.8619 151.85 68.9023 151.775 68.9603C151.721 68.9988 151.683 69.0549 151.665 69.119C151.648 69.1714 151.64 69.2262 151.64 69.2813C151.638 69.3239 151.645 69.3664 151.658 69.4068C151.675 69.4532 151.705 69.4931 151.746 69.5212C151.806 69.5661 151.873 69.601 151.944 69.6246C152.058 69.6631 152.174 69.6939 152.292 69.7168L153.069 69.8718C153.319 69.9168 153.564 69.9848 153.802 70.0748C153.991 70.1463 154.167 70.2472 154.326 70.3737C154.464 70.4892 154.573 70.6359 154.644 70.8018C154.719 70.992 154.755 71.1952 154.751 71.3997C154.76 71.6334 154.712 71.8658 154.612 72.0768C154.512 72.2877 154.362 72.4708 154.175 72.6101C153.694 72.9266 153.125 73.0793 152.552 73.0456C152.258 73.0468 151.965 73.0196 151.676 72.9644C151.439 72.9175 151.205 72.8546 150.976 72.7762C150.81 72.7177 150.648 72.6499 150.489 72.5732C150.411 72.5359 150.336 72.4915 150.265 72.4404L150.764 71.4661Z\" fill=\"black\"/>\n<path d=\"M157.308 71.0456C157.195 70.9053 157.074 70.7577 156.942 70.5953C156.81 70.4329 156.693 70.2743 156.575 70.1193L156.249 69.6875C156.147 69.5546 156.066 69.4439 156.004 69.3553C155.942 69.2668 155.901 69.2077 155.839 69.1265C155.778 69.0423 155.722 68.9548 155.67 68.8645C155.621 68.7781 155.578 68.688 155.542 68.5951C155.509 68.5182 155.492 68.4353 155.491 68.3515C155.487 68.2812 155.5 68.211 155.529 68.1467C155.557 68.0825 155.601 68.0262 155.656 67.9825C155.772 67.8992 155.912 67.8564 156.055 67.8607C156.173 67.8578 156.29 67.874 156.403 67.9087C156.465 67.9261 156.525 67.9496 156.583 67.9788L156.869 68.4143C156.986 68.5914 157.107 68.7833 157.235 68.9641C157.363 69.145 157.488 69.3332 157.601 69.514C157.715 69.6949 157.829 69.8388 157.917 69.9569C158.005 69.8388 158.096 69.7096 158.191 69.5878C158.287 69.466 158.382 69.3073 158.477 69.1597L158.756 68.7243C158.847 68.5803 158.928 68.4475 159.009 68.3257C159.038 68.2814 159.075 68.2297 159.115 68.1781C159.158 68.1208 159.21 68.0709 159.269 68.0304C159.335 67.9826 159.408 67.9453 159.485 67.9197C159.579 67.8881 159.679 67.8731 159.778 67.8754C159.862 67.8751 159.946 67.8863 160.027 67.9087C160.099 67.9288 160.169 67.9547 160.236 67.9862C160.289 68.0108 160.339 68.0404 160.386 68.0747L160.467 68.1412C160.427 68.2223 160.364 68.3368 160.284 68.477C160.203 68.6172 160.101 68.7722 159.987 68.9494L159.621 69.5066L159.225 70.086C159.093 70.2743 158.965 70.4551 158.84 70.6248C158.715 70.7946 158.606 70.9422 158.514 71.0714V72.9388H157.29L157.308 71.0456Z\" fill=\"black\"/>\n<path d=\"M163.67 68.3957C163.661 68.3158 163.673 68.2349 163.703 68.1607C163.734 68.0865 163.783 68.0213 163.846 67.9713C163.984 67.8843 164.145 67.8418 164.307 67.8495C164.431 67.8477 164.555 67.8651 164.674 67.9012L164.883 67.9676V70.551C164.88 70.748 164.899 70.9447 164.938 71.1377C164.967 71.2922 165.029 71.4383 165.121 71.5658C165.208 71.6823 165.324 71.7728 165.458 71.8279C165.797 71.946 166.166 71.946 166.506 71.8279C166.639 71.7719 166.756 71.6815 166.843 71.5658C166.933 71.4375 166.995 71.2917 167.026 71.1377C167.065 70.9447 167.084 70.748 167.081 70.551V68.4031C167.073 68.3232 167.084 68.2423 167.115 68.1681C167.146 68.0939 167.195 68.0287 167.257 67.9787C167.395 67.8917 167.556 67.8492 167.719 67.8569C167.843 67.855 167.967 67.8725 168.085 67.9086L168.294 67.975V70.6764C168.331 71.3166 168.115 71.9454 167.693 72.4257C167.209 72.8309 166.6 73.0528 165.971 73.0528C165.342 73.0528 164.732 72.8309 164.249 72.4257C164.039 72.1883 163.878 71.9116 163.775 71.6114C163.672 71.3112 163.629 70.9935 163.648 70.6764L163.67 68.3957Z\" fill=\"black\"/>\n<path d=\"M169.8 72.9129V68.3957C169.792 68.3158 169.803 68.2349 169.834 68.1607C169.865 68.0865 169.914 68.0213 169.976 67.9713C170.114 67.8843 170.275 67.8418 170.438 67.8495C170.562 67.8474 170.686 67.8649 170.804 67.9012L171.013 67.9676V72.9129H169.8Z\" fill=\"black\"/>\n<path d=\"M176.052 69.0117H174.879C174.707 69.0284 174.534 68.9767 174.399 68.8678C174.349 68.8114 174.31 68.7454 174.286 68.6738C174.261 68.6022 174.251 68.5263 174.256 68.4507C174.254 68.3252 174.274 68.2002 174.315 68.0817C174.339 68.0037 174.371 67.9283 174.41 67.8566H178.441C178.613 67.8399 178.785 67.8915 178.921 68.0005C178.971 68.0567 179.01 68.1226 179.035 68.1943C179.059 68.266 179.069 68.3419 179.064 68.4175C179.066 68.5431 179.046 68.6681 179.005 68.7866C178.98 68.8643 178.948 68.9397 178.91 69.0117H177.268V72.9236H176.044L176.052 69.0117Z\" fill=\"black\"/>\n<path d=\"M179.727 70.3809C179.724 70.0207 179.795 69.6638 179.936 69.3328C180.069 69.0148 180.265 68.7273 180.511 68.4877C180.76 68.2439 181.054 68.052 181.376 67.923C181.878 67.7151 182.428 67.6578 182.961 67.758C183.494 67.8582 183.987 68.1116 184.381 68.4877C184.759 68.8652 185.016 69.3485 185.119 69.8748C185.221 70.4012 185.165 70.9465 184.956 71.44C184.821 71.7567 184.625 72.0438 184.381 72.2852C183.86 72.786 183.166 73.0627 182.446 73.0565C182.079 73.0589 181.716 72.9887 181.376 72.8498C181.055 72.7181 180.762 72.5265 180.511 72.2852C180.261 72.0391 180.062 71.7445 179.927 71.4192C179.792 71.0938 179.724 70.7444 179.727 70.3919V70.3809ZM182.446 71.894C182.645 71.8949 182.841 71.8561 183.025 71.7796C183.196 71.7047 183.351 71.5955 183.479 71.4585C183.605 71.3171 183.706 71.1547 183.776 70.9787C183.918 70.5917 183.918 70.1664 183.776 69.7793C183.708 69.602 183.607 69.4392 183.479 69.2996C183.352 69.1625 183.197 69.0544 183.025 68.9822C182.654 68.8296 182.238 68.8296 181.867 68.9822C181.694 69.0534 181.539 69.1618 181.413 69.2996C181.282 69.4373 181.181 69.6006 181.116 69.7793C180.974 70.1664 180.974 70.5917 181.116 70.9787C181.184 71.1561 181.284 71.3189 181.413 71.4585C181.54 71.5963 181.695 71.7056 181.867 71.7796C182.05 71.8568 182.247 71.8958 182.446 71.894Z\" fill=\"black\"/>\n<path d=\"M186.173 70.3809C186.17 70.0207 186.241 69.6638 186.382 69.3328C186.515 69.0148 186.71 68.7273 186.957 68.4877C187.205 68.2439 187.499 68.052 187.822 67.923C188.323 67.7151 188.874 67.6578 189.407 67.758C189.94 67.8582 190.433 68.1116 190.826 68.4877C191.205 68.8652 191.462 69.3485 191.564 69.8748C191.667 70.4012 191.61 70.9465 191.402 71.44C191.266 71.7567 191.071 72.0438 190.826 72.2852C190.306 72.786 189.612 73.0627 188.892 73.0565C188.525 73.0589 188.161 72.9887 187.822 72.8498C187.501 72.7181 187.207 72.5265 186.957 72.2852C186.706 72.0391 186.508 71.7445 186.373 71.4192C186.238 71.0938 186.17 70.7444 186.173 70.3919V70.3809ZM188.892 71.894C189.09 71.8949 189.287 71.8561 189.471 71.7796C189.642 71.7047 189.797 71.5955 189.925 71.4585C190.051 71.3171 190.151 71.1547 190.222 70.9787C190.363 70.5917 190.363 70.1664 190.222 69.7793C190.154 69.602 190.053 69.4392 189.925 69.2996C189.798 69.1625 189.643 69.0544 189.471 68.9822C189.099 68.8296 188.684 68.8296 188.313 68.9822C188.14 69.0534 187.985 69.1618 187.858 69.2996C187.728 69.4373 187.627 69.6006 187.561 69.7793C187.42 70.1664 187.42 70.5917 187.561 70.9787C187.629 71.1561 187.73 71.3189 187.858 71.4585C187.986 71.5963 188.141 71.7056 188.313 71.7796C188.496 71.8568 188.693 71.8958 188.892 71.894Z\" fill=\"black\"/>\n<path d=\"M192.875 72.9129V68.3957C192.866 68.3158 192.878 68.2349 192.909 68.1607C192.939 68.0865 192.988 68.0213 193.051 67.9713C193.189 67.8843 193.35 67.8418 193.512 67.8495C193.636 67.8477 193.76 67.8651 193.879 67.9012L194.088 67.9676V71.7614H196.411V72.9129H192.875Z\" fill=\"black\"/>\n<path d=\"M197.408 72.9126V68.3955C197.399 68.3155 197.411 68.2347 197.441 68.1605C197.472 68.0863 197.521 68.0211 197.584 67.9711C197.722 67.884 197.883 67.8416 198.045 67.8493C198.169 67.8472 198.293 67.8646 198.412 67.901L198.621 67.9674V69.8458C198.734 69.7167 198.859 69.5764 198.987 69.4288L199.408 68.9749L199.826 68.5394C199.962 68.4029 200.086 68.2848 200.193 68.1704C200.287 68.0848 200.39 68.0093 200.5 67.9452C200.634 67.8684 200.786 67.8313 200.94 67.8382C201.044 67.8357 201.148 67.8494 201.248 67.8788C201.331 67.9013 201.411 67.9335 201.486 67.9748C201.545 68.0063 201.601 68.0447 201.651 68.0892C201.686 68.1173 201.718 68.1495 201.746 68.1851L201.578 68.3549L201.266 68.6686L200.874 69.0708L200.456 69.4989C200.321 69.6392 200.193 69.7683 200.09 69.8938C199.987 70.0193 199.885 70.1152 199.815 70.1964C200.02 70.4179 200.215 70.6356 200.401 70.8496C200.588 71.0637 200.768 71.2814 200.94 71.4992C201.112 71.7169 201.285 71.942 201.453 72.1708C201.622 72.3996 201.794 72.6432 201.97 72.9089H200.467C200.339 72.7355 200.2 72.5399 200.042 72.348C199.885 72.1561 199.727 71.9494 199.562 71.7501C199.397 71.5508 199.233 71.3589 199.068 71.1781C198.903 70.9973 198.749 70.8349 198.613 70.6983V72.9126H197.408Z\" fill=\"black\"/>\n<path d=\"M202.978 72.9129V68.3957C202.969 68.3158 202.98 68.2349 203.011 68.1607C203.042 68.0865 203.091 68.0213 203.153 67.9713C203.291 67.8843 203.453 67.8418 203.615 67.8495C203.739 67.8474 203.863 67.8649 203.982 67.9012L204.19 67.9676V72.9129H202.978Z\" fill=\"black\"/>\n<path d=\"M206.986 69.0117H205.814C205.641 69.0284 205.469 68.9767 205.334 68.8678C205.284 68.8114 205.245 68.7454 205.22 68.6738C205.196 68.6022 205.186 68.5263 205.191 68.4507C205.188 68.3252 205.208 68.2002 205.249 68.0817C205.274 68.0037 205.306 67.9283 205.345 67.8566H209.376C209.548 67.8399 209.72 67.8915 209.856 68.0005C209.906 68.0567 209.945 68.1226 209.969 68.1943C209.994 68.266 210.004 68.3419 209.998 68.4175C210.001 68.5431 209.981 68.6681 209.94 68.7866C209.915 68.8643 209.883 68.9397 209.845 69.0117H208.21V72.9236H206.986V69.0117Z\" fill=\"black\"/>\n<path d=\"M104.068 44.053C102.09 42.5571 99.317 41.573 95.7503 41.1006L89.1764 40.2592C87.4126 40.0402 86.0593 39.6392 85.1163 39.0561C84.6694 38.8013 84.3007 38.4275 84.0506 37.9758C83.8005 37.5241 83.6786 37.0118 83.6982 36.4949C83.6982 33.8525 86.19 32.5326 91.1735 32.535C94.0067 32.4954 96.8241 32.9679 99.4917 33.93C100.31 34.2196 101.098 34.5904 101.844 35.0372C102.243 35.2742 102.621 35.5458 102.973 35.8491C103.182 35.7534 103.377 35.6292 103.552 35.48C103.827 35.2668 104.073 35.0187 104.285 34.7419C104.533 34.4174 104.749 34.0688 104.93 33.7012C105.131 33.2765 105.23 32.81 105.219 32.3394C105.219 30.7378 103.984 29.4387 101.514 28.4423C99.0446 27.4459 95.5744 26.9538 91.1039 26.9661C88.8105 26.9191 86.5217 27.1924 84.3028 27.778C82.6536 28.2084 81.108 28.9704 79.759 30.0181C78.6613 30.889 77.7878 32.013 77.2122 33.2953C76.6733 34.5328 76.3986 35.8706 76.406 37.2219C76.3688 38.6426 76.6677 40.0518 77.2782 41.3331C77.85 42.4717 78.6767 43.4609 79.693 44.2228C80.8161 45.0455 82.0702 45.6695 83.4014 46.068C84.945 46.5426 86.5325 46.8579 88.1394 47.0091L93.1669 47.6069C95.4877 47.853 97.0988 48.2958 98.0003 48.9355C98.4417 49.2403 98.7979 49.6543 99.0347 50.1377C99.2715 50.6211 99.3809 51.1577 99.3524 51.696C99.3524 53.1254 98.644 54.1317 97.2271 54.7148C95.8102 55.2979 93.9841 55.5894 91.7488 55.5894C90.1528 55.599 88.5589 55.4693 86.9851 55.2019C85.6637 54.9712 84.3588 54.6531 83.0789 54.2498C82.1044 53.943 81.1466 53.5845 80.2097 53.1759C79.6485 52.9441 79.1089 52.6626 78.5974 52.3344L75.8271 57.3313C76.3529 57.7106 76.9137 58.0381 77.5017 58.3093C78.5975 58.8338 79.729 59.279 80.8876 59.6416C82.504 60.1492 84.1516 60.5499 85.8198 60.841C87.9047 61.2045 90.0178 61.3787 92.1336 61.3613C96.9486 61.3613 100.634 60.3871 103.189 58.4385C104.412 57.5544 105.401 56.382 106.07 55.0239C106.739 53.6658 107.067 52.1633 107.026 50.6479C107.026 47.7472 106.04 45.5489 104.068 44.053Z\" fill=\"black\"/>\n<path d=\"M119.027 12.5995C117.781 12.5995 116.828 12.8836 116.124 13.4446C115.421 14.0056 115.095 14.8728 115.095 16.0279V60.4428H122.442V13.3154C122.226 13.2269 121.797 13.0866 121.152 12.8947C120.462 12.6918 119.745 12.5924 119.027 12.5995Z\" fill=\"black\"/>\n<path d=\"M135.993 13.2489C135.408 13.243 134.829 13.3544 134.288 13.5766C133.747 13.7988 133.255 14.1274 132.842 14.5432C132.428 14.9589 132.101 15.4535 131.88 15.998C131.658 16.5425 131.546 17.1259 131.551 17.7144C131.534 18.3133 131.637 18.9096 131.852 19.468C132.068 20.0263 132.393 20.5354 132.808 20.9651C133.222 21.3949 133.718 21.7365 134.266 21.9697C134.814 22.203 135.403 22.3232 135.998 22.3232C136.593 22.3232 137.182 22.203 137.73 21.9697C138.278 21.7365 138.774 21.3949 139.189 20.9651C139.603 20.5354 139.928 20.0263 140.144 19.468C140.36 18.9096 140.462 18.3133 140.445 17.7144C140.45 17.125 140.338 16.5406 140.116 15.9954C139.893 15.4501 139.565 14.9551 139.151 14.5392C138.736 14.1233 138.243 13.7949 137.701 13.5734C137.159 13.3518 136.578 13.2415 135.993 13.2489Z\" fill=\"black\"/>\n<path d=\"M136.249 27.856C135.003 27.856 134.036 28.1266 133.347 28.6679C132.658 29.2091 132.315 30.085 132.317 31.2955V60.4502H139.664V28.5719C139.448 28.4833 139.019 28.3431 138.374 28.1475C137.684 27.9459 136.968 27.8476 136.249 27.856Z\" fill=\"black\"/>\n<path d=\"M176.206 30.631C174.736 29.3988 173.038 28.4719 171.211 27.9038C169.169 27.2529 167.039 26.9278 164.897 26.9406C162.766 26.9182 160.645 27.2358 158.613 27.8816C156.794 28.461 155.105 29.3952 153.644 30.631C152.243 31.8203 151.117 33.3042 150.346 34.9784C149.527 36.8035 149.121 38.7886 149.155 40.7909V60.4575H156.502V42.6583C156.502 39.7059 157.201 37.4658 158.598 35.9379C159.996 34.4101 162.095 33.6425 164.897 33.6351C167.646 33.6351 169.729 34.4027 171.149 35.9379C172.568 37.4732 173.278 39.7133 173.278 42.6583V60.4427H180.625V40.7761C180.658 38.777 180.262 36.7943 179.463 34.9636C178.712 33.2926 177.599 31.8122 176.206 30.631Z\" fill=\"black\"/>\n<path d=\"M207.628 53.1097C206.905 53.5786 206.126 53.9508 205.308 54.2168C204.19 54.6122 203.012 54.8095 201.827 54.7999C201.078 54.8033 200.331 54.7166 199.603 54.5416C198.92 54.381 198.282 54.0697 197.734 53.6301C197.177 53.1734 196.744 52.5816 196.477 51.9103C196.147 51.0304 195.994 50.0934 196.026 49.1535V34.6905H208.401C208.598 34.2569 208.771 33.812 208.918 33.3583C209.137 32.6934 209.245 31.9961 209.236 31.2953C209.236 30.1882 208.947 29.3172 208.364 28.7341C207.782 28.151 206.785 27.8558 205.367 27.8558H196.045V19.3677C195.828 19.2791 195.389 19.1389 194.722 18.9433C194.019 18.7452 193.293 18.6471 192.563 18.6517C191.554 18.5945 190.555 18.8804 189.727 19.4636C189.038 20.0061 188.697 20.8808 188.697 22.0912V50.6555C188.697 53.8539 189.664 56.4373 191.596 58.4055C193.528 60.3738 196.387 61.3579 200.171 61.3579C201.401 61.3622 202.629 61.2385 203.835 60.9889C204.866 60.775 205.879 60.4814 206.865 60.1105C207.662 59.8161 208.429 59.4454 209.156 59.0034C209.776 58.6171 210.265 58.2936 210.621 58.0328L207.628 53.1097Z\" fill=\"black\"/>\n<defs>\n<radialGradient id=\"paint0_radial_1014_7487\" cx=\"0\" cy=\"0\" r=\"1\" gradientUnits=\"userSpaceOnUse\" gradientTransform=\"translate(25.8302 24.1136) scale(86.2046 86.8185)\">\n<stop offset=\"0.15\" stop-color=\"#FBFCFF\"/>\n<stop offset=\"0.53\" stop-color=\"#F8F9FD\"/>\n<stop offset=\"0.82\" stop-color=\"#EEF0F8\"/>\n<stop offset=\"1\" stop-color=\"#E4E6F3\"/>\n</radialGradient>\n</defs>\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = :: core :: include_bytes ! ("/Users/liufankai/Desktop/code/ai/Promptpro/./logo.png") ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_0_0 = slint :: VersionCheck_1_0_0 ;
     }
 pub use slint_generatedApp :: {
     r#App , r#GeneratePageAdapter }
 ;
 pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
