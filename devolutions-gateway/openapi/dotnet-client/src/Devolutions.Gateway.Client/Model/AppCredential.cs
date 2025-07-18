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
    /// AppCredential
    /// </summary>
    [DataContract(Name = "AppCredential")]
    public partial class AppCredential : IValidatableObject
    {

        /// <summary>
        /// Gets or Sets Kind
        /// </summary>
        [DataMember(Name = "kind", IsRequired = true, EmitDefaultValue = true)]
        public AppCredentialKind Kind { get; set; }
        /// <summary>
        /// Initializes a new instance of the <see cref="AppCredential" /> class.
        /// </summary>
        [JsonConstructorAttribute]
        protected AppCredential() { }
        /// <summary>
        /// Initializes a new instance of the <see cref="AppCredential" /> class.
        /// </summary>
        /// <param name="kind">kind (required).</param>
        /// <param name="password">Password for the credential.  Required for \&quot;username-password\&quot; kind..</param>
        /// <param name="username">Username for the credential.  Required for \&quot;username-password\&quot; kind..</param>
        public AppCredential(AppCredentialKind kind = default(AppCredentialKind), string password = default(string), string username = default(string))
        {
            this.Kind = kind;
            this.Password = password;
            this.Username = username;
        }

        /// <summary>
        /// Password for the credential.  Required for \&quot;username-password\&quot; kind.
        /// </summary>
        /// <value>Password for the credential.  Required for \&quot;username-password\&quot; kind.</value>
        [DataMember(Name = "password", EmitDefaultValue = true)]
        public string Password { get; set; }

        /// <summary>
        /// Username for the credential.  Required for \&quot;username-password\&quot; kind.
        /// </summary>
        /// <value>Username for the credential.  Required for \&quot;username-password\&quot; kind.</value>
        [DataMember(Name = "username", EmitDefaultValue = true)]
        public string Username { get; set; }

        /// <summary>
        /// Returns the string presentation of the object
        /// </summary>
        /// <returns>String presentation of the object</returns>
        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            sb.Append("class AppCredential {\n");
            sb.Append("  Kind: ").Append(Kind).Append("\n");
            sb.Append("  Password: ").Append(Password).Append("\n");
            sb.Append("  Username: ").Append(Username).Append("\n");
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
