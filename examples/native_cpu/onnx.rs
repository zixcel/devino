//! Small synthetic ONNX graph: no framework, downloader, or converter runtime.
use openvino::{Core, DeviceType, ElementType, RwPropertyKey};
use std::error::Error;
pub fn verify_direct_onnx() -> Result<(), Box<dyn Error>> {
    // ONNX ModelProto: IR 8, opset 18, one Identity, static F32[2] input/output.
    const MODEL: &[u8] = b"\x08\x08:E\x0a\x10\x0a\x01x\x12\x01y\x22\x08Identity\x12\x0fdevino-identityZ\x0f\x0a\x01x\x12\x0a\x0a\x08\x08\x01\x12\x04\x0a\x02\x08\x02b\x0f\x0a\x01y\x12\x0a\x0a\x08\x08\x01\x12\x04\x0a\x02\x08\x02B\x02\x10\x12";
    let mut core = Core::new()?;
    core.set_property(&DeviceType::CPU, &RwPropertyKey::InferenceNumThreads, "1")?;
    core.set_property(&DeviceType::CPU, &RwPropertyKey::NumStreams, "1")?;
    let model = core.read_model_from_buffer(MODEL, None)?;
    let mut compiled = core.compile_model(&model, DeviceType::CPU)?;
    if compiled.get_input_size()? != 1 || compiled.get_output_size()? != 1 {
        return Err("unexpected synthetic ONNX ports".into());
    }
    let mut request = compiled.create_infer_request()?;
    for values in [[1.25_f32, -2.0], [0.0, 13.5]] {
        let mut input = request.get_input_tensor()?;
        if input.get_element_type()? != ElementType::F32 || input.get_size()? != 2 {
            return Err("unexpected synthetic ONNX input type/shape".into());
        }
        input.get_data_mut::<f32>()?.copy_from_slice(&values);
        request.infer()?;
        let output = request.get_output_tensor()?;
        if output.get_element_type()? != ElementType::F32 || output.get_size()? != 2 {
            return Err("unexpected synthetic ONNX output type/shape".into());
        }
        if output
            .get_data::<f32>()?
            .iter()
            .map(|v| v.to_bits())
            .collect::<Vec<_>>()
            != values.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        {
            return Err("ONNX frontend changed identity output".into());
        }
    }
    Ok(())
}
