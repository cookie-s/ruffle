use std::f64::consts::PI;

use crate::avm2::error::argument_error;
use crate::avm2::globals::slots::flash_geom_perspective_projection as pp_slots;
use crate::avm2::{Activation, Error, Object, TObject, Value};
use crate::display_object::TDisplayObject;
use crate::{avm2_stub_getter, avm2_stub_setter};

const DEG2RAD: f64 = PI / 180.0;

fn get_width<'gc>(activation: &mut Activation<'_, 'gc>, this: Object<'gc>) -> f64 {
    let dobj = this
        .get_slot(pp_slots::DISPLAY_OBJECT)
        .as_object()
        .and_then(|e| e.as_display_object());

    match dobj {
        // Not associated with any DO
        None => 500.0,
        // Stage's PerspectiveProjection
        Some(dobj) if dobj.as_stage().is_some() => 500.0,
        // Associated with other DO.
        Some(_dobj) => activation.context.stage.stage_size().0 as f64,
    }
}

pub fn get_field_of_view<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Value<'gc>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    avm2_stub_getter!(
        activation,
        "flash.geom.PerspectiveProjection",
        "fieldOfView"
    );

    let this = this.as_object().unwrap();

    let focal_length = this
        .get_slot(pp_slots::_FOCAL_LENGTH)
        .coerce_to_number(activation)?;

    let width = get_width(activation, this);
    let fov = f64::atan((width / 2.0) / focal_length) / DEG2RAD * 2.0;

    Ok(fov.into())
}

pub fn set_field_of_view<'gc>(
    activation: &mut Activation<'_, 'gc>,
    this: Value<'gc>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error<'gc>> {
    // TODO: This setter should update the associated displayObject when there is.
    avm2_stub_setter!(
        activation,
        "flash.geom.PerspectiveProjection",
        "fieldOfView"
    );
    let this = this.as_object().unwrap();

    let fov = args.get(0).unwrap().coerce_to_number(activation)?;
    if fov <= 0.0 || 180.0 <= fov {
        return Err(Error::AvmError(argument_error(
            activation,
            &format!("Error #2182: Invalid fieldOfView value.  The value must be greater than 0 and less than 180."),
            2182,
        )?));
    }

    let width = get_width(activation, this);
    let focal_length = (width / 2.0) as f32 * f64::tan((PI - fov * DEG2RAD) / 2.0) as f32;
    this.set_slot(pp_slots::_FOCAL_LENGTH, focal_length.into(), activation)?;

    Ok(Value::Undefined)
}
