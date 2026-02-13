use rtsyn_plugin::prelude::*;
use serde_json::Value;

#[derive(Debug)]
struct Izhikevich2003Neuron {
    i_syn: f64,
    v: f64,
    u: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    v_mv: f64,
}

impl Default for Izhikevich2003Neuron {
    fn default() -> Self {
        Self {
            i_syn: 0.0,
            v: -65.0,
            u: -13.0,
            a: 0.02,
            b: 0.2,
            c: -65.0,
            d: 8.0,
            v_mv: -65.0,
        }
    }
}

impl PluginDescriptor for Izhikevich2003Neuron {
    fn name() -> &'static str {
        "Izhikevich 2003 Neuron"
    }

    fn kind() -> &'static str {
        "izhikevich_2003_neuron"
    }

    fn plugin_type() -> PluginType {
        PluginType::Computational
    }

    fn inputs() -> &'static [&'static str] {
        &["i_syn"]
    }

    fn outputs() -> &'static [&'static str] {
        &["Membrane potential (V)", "Membrane potential (mV)"]
    }

    fn internal_variables() -> &'static [&'static str] {
        &["v", "u"]
    }

    fn default_vars() -> Vec<(&'static str, Value)> {
        vec![
            ("v", (-65.0).into()),
            ("u", (-13.0).into()),
            ("a", 0.02.into()),
            ("b", 0.2.into()),
            ("c", (-65.0).into()),
            ("d", 8.0.into()),
        ]
    }

    fn behavior() -> PluginBehavior {
        PluginBehavior {
            supports_start_stop: true,
            supports_restart: true,
            supports_apply: false,
            extendable_inputs: ExtendableInputs::None,
            loads_started: false,
            external_window: false,
            starts_expanded: true,
            start_requires_connected_inputs: Vec::new(),
            start_requires_connected_outputs: Vec::new(),
        }
    }
}

impl PluginRuntime for Izhikevich2003Neuron {
    fn set_config_value(&mut self, key: &str, value: &Value) {
        if let Some(v) = value.as_f64() {
            match key {
                "v" => self.v = v,
                "u" => self.u = v,
                "a" => self.a = v,
                "b" => self.b = v,
                "c" => self.c = v,
                "d" => self.d = v,
                _ => {}
            }
        }
    }

    fn set_input_value(&mut self, key: &str, v: f64) {
        match key {
            "i_syn" => self.i_syn = if v.is_finite() { v } else { 0.0 },
            _ => {}
        }
    }

    fn process_tick(&mut self, _tick: u64, period_seconds: f64) {
        if !period_seconds.is_finite() || period_seconds <= 0.0 {
            return;
        }

        // Izhikevich model equations are defined in ms.
        let mut remaining_ms = period_seconds * 1000.0;
        const MAX_STEP_MS: f64 = 0.5;

        while remaining_ms > 0.0 {
            let dt_ms = remaining_ms.min(MAX_STEP_MS);
            remaining_ms -= dt_ms;

            let v0 = self.v;
            let u0 = self.u;

            let dv = 0.04 * v0 * v0 + 5.0 * v0 + 140.0 - u0 + self.i_syn;
            let du = self.a * (self.b * v0 - u0);

            self.v = v0 + dt_ms * dv;
            self.u = u0 + dt_ms * du;

            if self.v >= 30.0 {
                self.v = self.c;
                self.u += self.d;
            }
        }

        self.v_mv = self.v;
    }

    fn get_output_value(&self, key: &str) -> f64 {
        match key {
            "Membrane potential (V)" => self.v_mv / 1000.0,
            "Membrane potential (mV)" => self.v_mv,
            _ => 0.0,
        }
    }

    fn get_internal_value(&self, key: &str) -> Option<f64> {
        match key {
            "v" => Some(self.v),
            "u" => Some(self.u),
            _ => None,
        }
    }
}

rtsyn_plugin::export_plugin!(Izhikevich2003Neuron);
