/*eslint-disable block-scoped-var, id-length, no-control-regex, no-magic-numbers, no-prototype-builtins, no-redeclare, no-shadow, no-var, sort-vars*/
(function($protobuf) {
    "use strict";

    // Common aliases
    var $Reader = $protobuf.Reader, $Writer = $protobuf.Writer, $util = $protobuf.util;
    
    // Exported root namespace
    var $root = $protobuf.roots["default"] || ($protobuf.roots["default"] = {});
    
    $root.monitor_web_socket = (function() {
    
        /**
         * Namespace monitor_web_socket.
         * @exports monitor_web_socket
         * @namespace
         */
        var monitor_web_socket = {};
    
        monitor_web_socket.ToServer = (function() {
    
            /**
             * Properties of a ToServer.
             * @memberof monitor_web_socket
             * @interface IToServer
             * @property {monitor_web_socket.IAuthenticateRequest|null} [authReq] ToServer authReq
             */
    
            /**
             * Constructs a new ToServer.
             * @memberof monitor_web_socket
             * @classdesc Represents a ToServer.
             * @implements IToServer
             * @constructor
             * @param {monitor_web_socket.IToServer=} [properties] Properties to set
             */
            function ToServer(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * ToServer authReq.
             * @member {monitor_web_socket.IAuthenticateRequest|null|undefined} authReq
             * @memberof monitor_web_socket.ToServer
             * @instance
             */
            ToServer.prototype.authReq = null;
    
            // OneOf field names bound to virtual getters and setters
            var $oneOfFields;
    
            /**
             * ToServer msg.
             * @member {"authReq"|undefined} msg
             * @memberof monitor_web_socket.ToServer
             * @instance
             */
            Object.defineProperty(ToServer.prototype, "msg", {
                get: $util.oneOfGetter($oneOfFields = ["authReq"]),
                set: $util.oneOfSetter($oneOfFields)
            });
    
            /**
             * Creates a new ToServer instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.IToServer=} [properties] Properties to set
             * @returns {monitor_web_socket.ToServer} ToServer instance
             */
            ToServer.create = function create(properties) {
                return new ToServer(properties);
            };
    
            /**
             * Encodes the specified ToServer message. Does not implicitly {@link monitor_web_socket.ToServer.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.IToServer} message ToServer message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToServer.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.authReq != null && Object.hasOwnProperty.call(message, "authReq"))
                    $root.monitor_web_socket.AuthenticateRequest.encode(message.authReq, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified ToServer message, length delimited. Does not implicitly {@link monitor_web_socket.ToServer.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.IToServer} message ToServer message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToServer.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a ToServer message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.ToServer} ToServer
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToServer.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.ToServer();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.authReq = $root.monitor_web_socket.AuthenticateRequest.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a ToServer message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.ToServer} ToServer
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToServer.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a ToServer message.
             * @function verify
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ToServer.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                var properties = {};
                if (message.authReq != null && message.hasOwnProperty("authReq")) {
                    properties.msg = 1;
                    {
                        var error = $root.monitor_web_socket.AuthenticateRequest.verify(message.authReq);
                        if (error)
                            return "authReq." + error;
                    }
                }
                return null;
            };
    
            /**
             * Creates a ToServer message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.ToServer} ToServer
             */
            ToServer.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.ToServer)
                    return object;
                var message = new $root.monitor_web_socket.ToServer();
                if (object.authReq != null) {
                    if (typeof object.authReq !== "object")
                        throw TypeError(".monitor_web_socket.ToServer.authReq: object expected");
                    message.authReq = $root.monitor_web_socket.AuthenticateRequest.fromObject(object.authReq);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a ToServer message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.ToServer
             * @static
             * @param {monitor_web_socket.ToServer} message ToServer
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ToServer.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (message.authReq != null && message.hasOwnProperty("authReq")) {
                    object.authReq = $root.monitor_web_socket.AuthenticateRequest.toObject(message.authReq, options);
                    if (options.oneofs)
                        object.msg = "authReq";
                }
                return object;
            };
    
            /**
             * Converts this ToServer to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.ToServer
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ToServer.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return ToServer;
        })();
    
        monitor_web_socket.AuthenticateRequest = (function() {
    
            /**
             * Properties of an AuthenticateRequest.
             * @memberof monitor_web_socket
             * @interface IAuthenticateRequest
             * @property {string|null} [key] AuthenticateRequest key
             */
    
            /**
             * Constructs a new AuthenticateRequest.
             * @memberof monitor_web_socket
             * @classdesc Represents an AuthenticateRequest.
             * @implements IAuthenticateRequest
             * @constructor
             * @param {monitor_web_socket.IAuthenticateRequest=} [properties] Properties to set
             */
            function AuthenticateRequest(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * AuthenticateRequest key.
             * @member {string} key
             * @memberof monitor_web_socket.AuthenticateRequest
             * @instance
             */
            AuthenticateRequest.prototype.key = "";
    
            /**
             * Creates a new AuthenticateRequest instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {monitor_web_socket.IAuthenticateRequest=} [properties] Properties to set
             * @returns {monitor_web_socket.AuthenticateRequest} AuthenticateRequest instance
             */
            AuthenticateRequest.create = function create(properties) {
                return new AuthenticateRequest(properties);
            };
    
            /**
             * Encodes the specified AuthenticateRequest message. Does not implicitly {@link monitor_web_socket.AuthenticateRequest.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {monitor_web_socket.IAuthenticateRequest} message AuthenticateRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AuthenticateRequest.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.key != null && Object.hasOwnProperty.call(message, "key"))
                    writer.uint32(/* id 1, wireType 2 =*/10).string(message.key);
                return writer;
            };
    
            /**
             * Encodes the specified AuthenticateRequest message, length delimited. Does not implicitly {@link monitor_web_socket.AuthenticateRequest.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {monitor_web_socket.IAuthenticateRequest} message AuthenticateRequest message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AuthenticateRequest.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes an AuthenticateRequest message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.AuthenticateRequest} AuthenticateRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AuthenticateRequest.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.AuthenticateRequest();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.key = reader.string();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes an AuthenticateRequest message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.AuthenticateRequest} AuthenticateRequest
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AuthenticateRequest.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies an AuthenticateRequest message.
             * @function verify
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            AuthenticateRequest.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.key != null && message.hasOwnProperty("key"))
                    if (!$util.isString(message.key))
                        return "key: string expected";
                return null;
            };
    
            /**
             * Creates an AuthenticateRequest message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.AuthenticateRequest} AuthenticateRequest
             */
            AuthenticateRequest.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.AuthenticateRequest)
                    return object;
                var message = new $root.monitor_web_socket.AuthenticateRequest();
                if (object.key != null)
                    message.key = String(object.key);
                return message;
            };
    
            /**
             * Creates a plain object from an AuthenticateRequest message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.AuthenticateRequest
             * @static
             * @param {monitor_web_socket.AuthenticateRequest} message AuthenticateRequest
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            AuthenticateRequest.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults)
                    object.key = "";
                if (message.key != null && message.hasOwnProperty("key"))
                    object.key = message.key;
                return object;
            };
    
            /**
             * Converts this AuthenticateRequest to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.AuthenticateRequest
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            AuthenticateRequest.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return AuthenticateRequest;
        })();
    
        monitor_web_socket.ToClient = (function() {
    
            /**
             * Properties of a ToClient.
             * @memberof monitor_web_socket
             * @interface IToClient
             * @property {monitor_web_socket.IAuthenticateResponse|null} [authResp] ToClient authResp
             */
    
            /**
             * Constructs a new ToClient.
             * @memberof monitor_web_socket
             * @classdesc Represents a ToClient.
             * @implements IToClient
             * @constructor
             * @param {monitor_web_socket.IToClient=} [properties] Properties to set
             */
            function ToClient(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * ToClient authResp.
             * @member {monitor_web_socket.IAuthenticateResponse|null|undefined} authResp
             * @memberof monitor_web_socket.ToClient
             * @instance
             */
            ToClient.prototype.authResp = null;
    
            // OneOf field names bound to virtual getters and setters
            var $oneOfFields;
    
            /**
             * ToClient msg.
             * @member {"authResp"|undefined} msg
             * @memberof monitor_web_socket.ToClient
             * @instance
             */
            Object.defineProperty(ToClient.prototype, "msg", {
                get: $util.oneOfGetter($oneOfFields = ["authResp"]),
                set: $util.oneOfSetter($oneOfFields)
            });
    
            /**
             * Creates a new ToClient instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.IToClient=} [properties] Properties to set
             * @returns {monitor_web_socket.ToClient} ToClient instance
             */
            ToClient.create = function create(properties) {
                return new ToClient(properties);
            };
    
            /**
             * Encodes the specified ToClient message. Does not implicitly {@link monitor_web_socket.ToClient.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.IToClient} message ToClient message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToClient.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.authResp != null && Object.hasOwnProperty.call(message, "authResp"))
                    $root.monitor_web_socket.AuthenticateResponse.encode(message.authResp, writer.uint32(/* id 1, wireType 2 =*/10).fork()).ldelim();
                return writer;
            };
    
            /**
             * Encodes the specified ToClient message, length delimited. Does not implicitly {@link monitor_web_socket.ToClient.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.IToClient} message ToClient message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            ToClient.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes a ToClient message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.ToClient} ToClient
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToClient.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.ToClient();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.authResp = $root.monitor_web_socket.AuthenticateResponse.decode(reader, reader.uint32());
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes a ToClient message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.ToClient} ToClient
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            ToClient.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies a ToClient message.
             * @function verify
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            ToClient.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                var properties = {};
                if (message.authResp != null && message.hasOwnProperty("authResp")) {
                    properties.msg = 1;
                    {
                        var error = $root.monitor_web_socket.AuthenticateResponse.verify(message.authResp);
                        if (error)
                            return "authResp." + error;
                    }
                }
                return null;
            };
    
            /**
             * Creates a ToClient message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.ToClient} ToClient
             */
            ToClient.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.ToClient)
                    return object;
                var message = new $root.monitor_web_socket.ToClient();
                if (object.authResp != null) {
                    if (typeof object.authResp !== "object")
                        throw TypeError(".monitor_web_socket.ToClient.authResp: object expected");
                    message.authResp = $root.monitor_web_socket.AuthenticateResponse.fromObject(object.authResp);
                }
                return message;
            };
    
            /**
             * Creates a plain object from a ToClient message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.ToClient
             * @static
             * @param {monitor_web_socket.ToClient} message ToClient
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            ToClient.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (message.authResp != null && message.hasOwnProperty("authResp")) {
                    object.authResp = $root.monitor_web_socket.AuthenticateResponse.toObject(message.authResp, options);
                    if (options.oneofs)
                        object.msg = "authResp";
                }
                return object;
            };
    
            /**
             * Converts this ToClient to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.ToClient
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            ToClient.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return ToClient;
        })();
    
        monitor_web_socket.AuthenticateResponse = (function() {
    
            /**
             * Properties of an AuthenticateResponse.
             * @memberof monitor_web_socket
             * @interface IAuthenticateResponse
             * @property {boolean|null} [ok] AuthenticateResponse ok
             */
    
            /**
             * Constructs a new AuthenticateResponse.
             * @memberof monitor_web_socket
             * @classdesc Represents an AuthenticateResponse.
             * @implements IAuthenticateResponse
             * @constructor
             * @param {monitor_web_socket.IAuthenticateResponse=} [properties] Properties to set
             */
            function AuthenticateResponse(properties) {
                if (properties)
                    for (var keys = Object.keys(properties), i = 0; i < keys.length; ++i)
                        if (properties[keys[i]] != null)
                            this[keys[i]] = properties[keys[i]];
            }
    
            /**
             * AuthenticateResponse ok.
             * @member {boolean} ok
             * @memberof monitor_web_socket.AuthenticateResponse
             * @instance
             */
            AuthenticateResponse.prototype.ok = false;
    
            /**
             * Creates a new AuthenticateResponse instance using the specified properties.
             * @function create
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {monitor_web_socket.IAuthenticateResponse=} [properties] Properties to set
             * @returns {monitor_web_socket.AuthenticateResponse} AuthenticateResponse instance
             */
            AuthenticateResponse.create = function create(properties) {
                return new AuthenticateResponse(properties);
            };
    
            /**
             * Encodes the specified AuthenticateResponse message. Does not implicitly {@link monitor_web_socket.AuthenticateResponse.verify|verify} messages.
             * @function encode
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {monitor_web_socket.IAuthenticateResponse} message AuthenticateResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AuthenticateResponse.encode = function encode(message, writer) {
                if (!writer)
                    writer = $Writer.create();
                if (message.ok != null && Object.hasOwnProperty.call(message, "ok"))
                    writer.uint32(/* id 1, wireType 0 =*/8).bool(message.ok);
                return writer;
            };
    
            /**
             * Encodes the specified AuthenticateResponse message, length delimited. Does not implicitly {@link monitor_web_socket.AuthenticateResponse.verify|verify} messages.
             * @function encodeDelimited
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {monitor_web_socket.IAuthenticateResponse} message AuthenticateResponse message or plain object to encode
             * @param {$protobuf.Writer} [writer] Writer to encode to
             * @returns {$protobuf.Writer} Writer
             */
            AuthenticateResponse.encodeDelimited = function encodeDelimited(message, writer) {
                return this.encode(message, writer).ldelim();
            };
    
            /**
             * Decodes an AuthenticateResponse message from the specified reader or buffer.
             * @function decode
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @param {number} [length] Message length if known beforehand
             * @returns {monitor_web_socket.AuthenticateResponse} AuthenticateResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AuthenticateResponse.decode = function decode(reader, length) {
                if (!(reader instanceof $Reader))
                    reader = $Reader.create(reader);
                var end = length === undefined ? reader.len : reader.pos + length, message = new $root.monitor_web_socket.AuthenticateResponse();
                while (reader.pos < end) {
                    var tag = reader.uint32();
                    switch (tag >>> 3) {
                    case 1:
                        message.ok = reader.bool();
                        break;
                    default:
                        reader.skipType(tag & 7);
                        break;
                    }
                }
                return message;
            };
    
            /**
             * Decodes an AuthenticateResponse message from the specified reader or buffer, length delimited.
             * @function decodeDelimited
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {$protobuf.Reader|Uint8Array} reader Reader or buffer to decode from
             * @returns {monitor_web_socket.AuthenticateResponse} AuthenticateResponse
             * @throws {Error} If the payload is not a reader or valid buffer
             * @throws {$protobuf.util.ProtocolError} If required fields are missing
             */
            AuthenticateResponse.decodeDelimited = function decodeDelimited(reader) {
                if (!(reader instanceof $Reader))
                    reader = new $Reader(reader);
                return this.decode(reader, reader.uint32());
            };
    
            /**
             * Verifies an AuthenticateResponse message.
             * @function verify
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {Object.<string,*>} message Plain object to verify
             * @returns {string|null} `null` if valid, otherwise the reason why it is not
             */
            AuthenticateResponse.verify = function verify(message) {
                if (typeof message !== "object" || message === null)
                    return "object expected";
                if (message.ok != null && message.hasOwnProperty("ok"))
                    if (typeof message.ok !== "boolean")
                        return "ok: boolean expected";
                return null;
            };
    
            /**
             * Creates an AuthenticateResponse message from a plain object. Also converts values to their respective internal types.
             * @function fromObject
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {Object.<string,*>} object Plain object
             * @returns {monitor_web_socket.AuthenticateResponse} AuthenticateResponse
             */
            AuthenticateResponse.fromObject = function fromObject(object) {
                if (object instanceof $root.monitor_web_socket.AuthenticateResponse)
                    return object;
                var message = new $root.monitor_web_socket.AuthenticateResponse();
                if (object.ok != null)
                    message.ok = Boolean(object.ok);
                return message;
            };
    
            /**
             * Creates a plain object from an AuthenticateResponse message. Also converts values to other types if specified.
             * @function toObject
             * @memberof monitor_web_socket.AuthenticateResponse
             * @static
             * @param {monitor_web_socket.AuthenticateResponse} message AuthenticateResponse
             * @param {$protobuf.IConversionOptions} [options] Conversion options
             * @returns {Object.<string,*>} Plain object
             */
            AuthenticateResponse.toObject = function toObject(message, options) {
                if (!options)
                    options = {};
                var object = {};
                if (options.defaults)
                    object.ok = false;
                if (message.ok != null && message.hasOwnProperty("ok"))
                    object.ok = message.ok;
                return object;
            };
    
            /**
             * Converts this AuthenticateResponse to JSON.
             * @function toJSON
             * @memberof monitor_web_socket.AuthenticateResponse
             * @instance
             * @returns {Object.<string,*>} JSON object
             */
            AuthenticateResponse.prototype.toJSON = function toJSON() {
                return this.constructor.toObject(this, $protobuf.util.toJSONOptions);
            };
    
            return AuthenticateResponse;
        })();
    
        return monitor_web_socket;
    })();

    return $root;
})(protobuf);
