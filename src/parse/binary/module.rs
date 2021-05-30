use super::preamble;
use super::sections;
use super::{Parse, ParseError, ParsingData};
use crate::wasm::func::Func;
use crate::wasm::module::Module;
use crate::wasm::values::U32;
use sections::{
    code::CodeSection, custom::CustomSection, data::DataSection, elem::ElemSection,
    export::ExportSection, func::FuncSection, global::GlobalSection, import::ImportSection,
    mem::MemSection, start::StartSection, table::TableSection, types::TypeSection, Section,
};

pub struct BinaryModule {
    pub header: preamble::Preamble,
    pub custom: Vec<CustomSection>,
    pub types: TypeSection,
    pub imports: ImportSection,
    pub functions: FuncSection,
    pub tables: TableSection,
    pub mems: MemSection,
    pub globals: GlobalSection,
    pub exports: ExportSection,
    pub start: StartSection,
    pub elems: ElemSection,
    pub code: CodeSection,
    pub data: DataSection,
}

impl BinaryModule {
    pub fn new(
        header: preamble::Preamble,
        sections: Vec<Section>,
    ) -> Result<BinaryModule, &'static str> {
        let mut custom: Vec<CustomSection> = Vec::new();
        let mut types: Option<TypeSection> = None;
        let mut imports: Option<ImportSection> = None;
        let mut functions: Option<FuncSection> = None;
        let mut tables: Option<TableSection> = None;
        let mut mems: Option<MemSection> = None;
        let mut globals: Option<GlobalSection> = None;
        let mut exports: Option<ExportSection> = None;
        let mut start: Option<StartSection> = None;
        let mut elems: Option<ElemSection> = None;
        let mut code: Option<CodeSection> = None;
        let mut data: Option<DataSection> = None;
        let mut data_count: Option<U32> = None;

        for section in sections {
            println!("Got section: {:?}", section);
            match section {
                Section::Custom(s) => custom.push(s),
                Section::Type(s) => types = Some(s),
                Section::Import(s) => imports = Some(s),
                Section::Function(s) => functions = Some(s),
                Section::Table(s) => tables = Some(s),
                Section::Memory(s) => mems = Some(s),
                Section::Global(s) => globals = Some(s),
                Section::Export(s) => exports = Some(s),
                Section::Start(s) => start = Some(s),
                Section::Element(s) => elems = Some(s),
                Section::Code(s) => code = Some(s),
                Section::Data(s) => data = Some(s),
                Section::DataCount(s) => data_count = Some(s),
            }
        }

        let module = BinaryModule {
            header,
            custom,
            types: types.unwrap_or_default(),
            imports: imports.unwrap_or_default(),
            functions: functions.unwrap_or_default(),
            tables: tables.unwrap_or_default(),
            mems: mems.unwrap_or_default(),
            globals: globals.unwrap_or_default(),
            exports: exports.unwrap_or_default(),
            start: start.unwrap_or_default(),
            elems: elems.unwrap_or_default(),
            code: code.unwrap_or_default(),
            data: data.unwrap_or_default(),
        };

        if let Some(len) = data_count {
            if module.data.data.len() != *len as usize {
                return Err(
                    "Malformed module. Data section should have a length equal to that of data_count.",
                );
            }
        }

        if module.functions.funcs.len() != module.code.code.len() {
            return Err("Malformed module. Func and Code sections should have the same length.");
        }
        Ok(module)
    }
}

impl Parse for BinaryModule {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let header =
            preamble::Preamble::parse(data).map_err(|err| err.extend("Can't parse header"))?;
        let mut sections: Vec<Section> = Vec::new();
        while !data.is_empty() {
            sections
                .push(Section::parse(data).map_err(|err| err.extend("Couldn't parse section"))?);
        }

        Ok(BinaryModule::new(header, sections)
            .map_err(|err| ParseError::new(data, err.to_string()))?)
    }
}

impl Parse for Module {
    fn parse(data: &mut ParsingData) -> Result<Self, ParseError> {
        let binary = BinaryModule::parse(data).map_err(|err| err.extend("Can't binary module"))?;
        Ok(binary.into())
    }
}

impl From<BinaryModule> for Module {
    fn from(from: BinaryModule) -> Module {
        let funcs = from
            .functions
            .funcs
            .iter()
            .zip(from.code.code)
            .map(|(f, cd)| Func {
                index: *f,
                locals: cd.locals,
                body: cd.code,
            })
            .collect();
        Module {
            types: from.types.types,
            funcs: funcs,
            tables: from.tables.tables,
            mems: from.mems.mems,
            globals: from.globals.globals,
            elems: from.elems.seg,
            datas: from.data.data,
            start: from.start.start,
            imports: from.imports.imports,
            exports: from.exports.exports,
        }
    }
}
