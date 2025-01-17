export default {
  rpc: {
    getLatestSchemaId: {
      description:
        "Get the most recent (aka highest) Schema Id. Useful for then retrieving a list of all Schemas (1-[result])",
      params: [
        {
          name: "at",
          type: "BlockHash",
          isOptional: true,
        },
      ],
      type: "SchemaId",
    },
    getBySchemaId: {
      description: "Get a Schema by Id",
      params: [
        {
          name: "schema_id",
          type: "SchemaId",
        },
      ],
      type: "Option<SchemaResponse>",
    },
    checkSchemaValidity: {
      description: "",
      params: [
        {
          name: "model",
          type: "SchemaModel",
        },
        {
          name: "at",
          type: "BlockHash",
          isOptional: true,
        },
      ],
      type: "bool",
    },
  },
  types: {
    SchemaId: "u16",
    SchemaModel: "Vec<u8>",
    SchemaResponse: {
      schema_id: "SchemaId",
      model: "SchemaModel",
      model_type: "ModelType",
      payload_location: "PayloadLocation",
    },
    ModelType: {
      _enum: ["AvroBinary"],
    },
    PayloadLocation: {
      _enum: ["OnChain", "IPFS"],
    },
  },
};
