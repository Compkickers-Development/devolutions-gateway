/*
 * devolutions-gateway
 *
 * Protocol-aware fine-grained relay server
 *
 * The version of the OpenAPI document: 2025.2.1
 * Contact: infos@devolutions.net
 * Generated by: https://github.com/openapitools/openapi-generator.git
 */


using System;
using System.Collections;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.IO;
using System.Runtime.Serialization;
using System.Text;
using System.Text.RegularExpressions;
using Newtonsoft.Json;
using Newtonsoft.Json.Converters;
using Newtonsoft.Json.Linq;
using System.ComponentModel.DataAnnotations;
using FileParameter = Devolutions.Gateway.Client.Client.FileParameter;
using OpenAPIDateConverter = Devolutions.Gateway.Client.Client.OpenAPIDateConverter;

namespace Devolutions.Gateway.Client.Model
{
    /// <summary>
    /// SubProvisionerKey
    /// </summary>
    [DataContract(Name = "SubProvisionerKey")]
    public partial class SubProvisionerKey : IValidatableObject
    {

        /// <summary>
        /// Gets or Sets Encoding
        /// </summary>
        [DataMember(Name = "Encoding", EmitDefaultValue = true)]
        public DataEncoding? Encoding { get; set; }

        /// <summary>
        /// Gets or Sets Format
        /// </summary>
        [DataMember(Name = "Format", EmitDefaultValue = true)]
        public PubKeyFormat? Format { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="SubProvisionerKey" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected SubProvisionerKey() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="SubProvisionerKey" /> class.
        /// </summary>
        /// <param name="encoding">encoding.</param>
        /// <param name="format">format.</param>
        /// <param name="id">The key ID for this subkey (required).</param>
        /// <param name="value">The binary-to-text-encoded key data (required).</param>
        public SubProvisionerKey(DataEncoding? encoding = default(DataEncoding?), PubKeyFormat? format = default(PubKeyFormat?), string id = default(string), string value = default(string))
        {
            // to ensure "id" is required (not null)
            if (id == null)
            {
                throw new ArgumentNullException("id is a required property for SubProvisionerKey and cannot be null");
            }
            this.Id = id;
            // to ensure "value" is required (not null)
            if (value == null)
            {
                throw new ArgumentNullException("value is a required property for SubProvisionerKey and cannot be null");
            }
            this.Value = value;
            this.Encoding = encoding;
            this.Format = format;
        }

        /// <summary>
        /// The key ID for this subkey
        /// </summary>
        /// <value>The key ID for this subkey</value>
        [DataMember(Name = "Id", IsRequired = true, EmitDefaultValue = true)]
        public string Id { get; set; }

        /// <summary>
        /// The binary-to-text-encoded key data
        /// </summary>
        /// <value>The binary-to-text-encoded key data</value>
        [DataMember(Name = "Value", IsRequired = true, EmitDefaultValue = true)]
        public string Value { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class SubProvisionerKey {\n");
            sb.Append("  Encoding: ").Append(Encoding).Append("\n");
            sb.Append("  Format: ").Append(Format).Append("\n");
            sb.Append("  Id: ").Append(Id).Append("\n");
            sb.Append("  Value: ").Append(Value).Append("\n");
            sb.Append("}\n");
            return sb.ToString();
        }

        /// <summary>
        /// Returns the JSON string presentation of the object
        /// </summary>
        /// <returns>JSON string presentation of the object</returns>
        public virtual string ToJson()
        {
            return Newtonsoft.Json.JsonConvert.SerializeObject(this, Newtonsoft.Json.Formatting.Indented);
        }

        /// <summary>
        /// To validate all properties of the instance
        /// </summary>
        /// <param name="validationContext">Validation context</param>
        /// <returns>Validation Result</returns>
        IEnumerable<ValidationResult> IValidatableObject.Validate(ValidationContext validationContext)
        {
            yield break;
        }
    }

}
