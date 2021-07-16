import React, { Fragment, useState } from "react";
import PropTypes from "prop-types";
import { Menu, Transition } from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/solid";
function LightHeaderB(props) {
  const [state, setState] = useState({ dropdown: "Red" });
  function classNames(...classes) {
    return classes.filter(Boolean).join(" ");
  }

  return (
    <header className="text-gray-600 body-font shadow-sm">
      <div className=" flex flex-wrap p-5 flex-col md:flex-row items-center">
        <a
          href="/"
          className="flex title-font font-medium items-center text-gray-900 mb-4 md:mb-0"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            stroke="currentColor"
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth="2"
            className={`w-10 h-10 text-white p-2 bg-${props.theme}-500 rounded-full`}
            viewBox="0 0 24 24"
          >
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
          </svg>
          <span className="ml-3 text-xl">LatinArt</span>
        </a>
        <nav className="md:mr-auto md:ml-4 md:py-1 md:pl-4 md:border-l md:border-gray-400	flex flex-wrap items-center text-base justify-center">
          <a href="/galeria" className="mr-5 hover:text-gray-900">
            Galeria
          </a>
          <a href="/minar" className="mr-5 hover:text-gray-900">
            Minar
          </a>
          <a href="/mis_nfts" className="mr-5 hover:text-gray-900">
            Mis Nfts
          </a>
        </nav>
        <Menu as="div" className="relative inline-block text-left">
          {({ open }) => (
            <>
              <div>
                <Menu.Button className="inline-flex justify-center w-full rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-sm font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-100 focus:ring-yellow-500">
                  {state.dropdown}
                  <ChevronDownIcon
                    className="-mr-1 ml-2 h-5 w-5"
                    aria-hidden="true"
                  />
                </Menu.Button>
              </div>

              <Transition
                show={open}
                as={Fragment}
                enter="transition ease-out duration-100"
                enterFrom="transform opacity-0 scale-95"
                enterTo="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leaveFrom="transform opacity-100 scale-100"
                leaveTo="transform opacity-0 scale-95"
              >
                <Menu.Items
                  static
                  className="origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white outline-none"
                >
                  <div className="py-1">
                    <Menu.Item
                      onClick={() => {
                        setState({ dropdown: "Aurora" });
                      }}
                    >
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active
                              ? "bg-gray-100 text-gray-900"
                              : "text-gray-700",
                            "block px-2 py-2 text-sm text-center"
                          )}
                        >
                          Aurora
                        </a>
                      )}
                    </Menu.Item>
                    <Menu.Item
                      onClick={() => {
                        setState({ dropdown: "Near" });
                      }}
                    >
                      {({ active }) => (
                        <a
                          href="#"
                          className={classNames(
                            active
                              ? "bg-gray-100 text-gray-900"
                              : "text-gray-700",
                            "block px-2 py-2 text-sm text-center"
                          )}
                        >
                          Near
                        </a>
                      )}
                    </Menu.Item>
                  </div>
                </Menu.Items>
              </Transition>
            </>
          )}
        </Menu>
      </div>
    </header>
  );
}

LightHeaderB.defaultProps = {
  theme: "indigo",
};

LightHeaderB.propTypes = {
  theme: PropTypes.string.isRequired,
};

export default LightHeaderB;
